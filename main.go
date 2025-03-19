package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"sync"
)

// PathInfo struct to hold path and depth
type PathInfo struct {
	Path  string
	Depth int
}

func main() {
	var wg sync.WaitGroup
	var mu sync.Mutex
	activePaths := 0 // Counter to track active paths

	// Command-line flags
	var recursive bool
	var depth int
	var debug bool
	var absolutePaths bool
	flag.BoolVar(&recursive, "r", false, "Enable recursive search")
	flag.IntVar(&depth, "depth", 5, "Depth for recursive search")
	flag.BoolVar(&debug, "debug", false, "Show debug information about buffer usage")
	flag.BoolVar(&absolutePaths, "a", false, "Print absolute paths for Git projects")
	flag.Parse()

	// Determine initial path
	initialPathArg := "."
	if len(flag.Args()) > 0 {
		initialPathArg = flag.Arg(0)
	}
	initialPath, err := filepath.Abs(initialPathArg)
	if err != nil {
		fmt.Printf("Error determining initial path: %v\n", err)
		os.Exit(1)
	}

	// Set depth based on recursive flag
	if !recursive {
		depth = 1
	}

	// Channel buffer sizes
	const pathsBufferSize = 1000000
	const foundPathsBufferSize = 1000000

	message := make(chan string) // Unbuffered channel for messages
	paths := make(chan PathInfo, pathsBufferSize)
	foundPaths := make(chan PathInfo, foundPathsBufferSize)

	// Variables to track maximum buffer usage
	var maxPathsUsage, maxFoundPathsUsage int
	var usageMu sync.Mutex

	// Counter for Git projects found
	var gitProjectsFound int
	var gitProjectsMu sync.Mutex

	// Start the printer goroutine
	wg.Add(1)
	go printer(&wg, message)

	// Start one worker per CPU
	numWorkers := runtime.NumCPU()
	for range numWorkers {
		wg.Add(1)
		go worker(&wg, &mu, &activePaths, paths, foundPaths, message, &usageMu, &maxPathsUsage, &maxFoundPathsUsage, &gitProjectsFound, &gitProjectsMu, initialPath, absolutePaths)
	}

	// Start the manager goroutine
	wg.Add(1)
	go manager(&wg, &mu, &activePaths, paths, foundPaths, PathInfo{Path: initialPath, Depth: depth}, &usageMu, &maxPathsUsage)

	wg.Wait() // Wait for all workers to finish

	// Print debug information if debug flag is enabled
	if debug {
		fmt.Printf("Max paths buffer usage: %d / %d\n", maxPathsUsage, pathsBufferSize)
		fmt.Printf("Max foundPaths buffer usage: %d / %d\n", maxFoundPathsUsage, foundPathsBufferSize)
		fmt.Printf("Total Git projects found: %d\n", gitProjectsFound)
	}
}

func printer(wg *sync.WaitGroup, message <-chan string) {
	defer wg.Done()
	for msg := range message {
		fmt.Println(msg)
	}
}

func worker(wg *sync.WaitGroup, mu *sync.Mutex, activePaths *int, paths <-chan PathInfo, foundPaths chan<- PathInfo, message chan<- string, usageMu *sync.Mutex, maxPathsUsage *int, maxFoundPathsUsage *int, gitProjectsFound *int, gitProjectsMu *sync.Mutex, initialPath string, absolutePaths bool) {
	defer wg.Done()

	for pathInfo := range paths {
		path := pathInfo.Path
		depth := pathInfo.Depth

		// Skip processing if the current path is a .git directory
		if filepath.Base(path) == ".git" {
			mu.Lock()
			*activePaths-- // Decrement active paths for skipped .git directory
			mu.Unlock()
			continue
		}

		subfolders, err := os.ReadDir(path)
		if err != nil {
			mu.Lock()
			*activePaths-- // Decrement active paths on error
			mu.Unlock()
			continue
		}

		isGitRepo := false
		for _, folder := range subfolders {
			// If the folder is a .git directory or a HEAD file, mark the path as a Git repository
			if folder.IsDir() && folder.Name() == ".git" {
				isGitRepo = true
				break
			}
			if !folder.IsDir() && folder.Name() == "HEAD" {
				isGitRepo = true
				break
			}
		}

		if isGitRepo {
			// Determine whether to print relative or absolute path
			var outputPath string
			if absolutePaths {
				outputPath = path
			} else {
				relativePath, err := filepath.Rel(initialPath, path)
				if err != nil {
					outputPath = path // Fallback to absolute path if relative path calculation fails
				} else {
					outputPath = relativePath
				}
			}
			message <- outputPath

			// Increment Git projects found counter
			gitProjectsMu.Lock()
			*gitProjectsFound++
			gitProjectsMu.Unlock()
		}

		// If depth > 0, process subfolders
		if depth > 0 {
			for _, folder := range subfolders {
				if folder.IsDir() {
					foundPaths <- PathInfo{Path: filepath.Join(path, folder.Name()), Depth: depth - 1}
					mu.Lock()
					*activePaths++ // Increment active paths for each new folder
					mu.Unlock()

					// Track maximum foundPaths buffer usage
					usageMu.Lock()
					if len(foundPaths) > *maxFoundPathsUsage {
						*maxFoundPathsUsage = len(foundPaths)
					}
					usageMu.Unlock()
				}
			}
		}

		// Decrement active paths after processing the current path
		mu.Lock()
		*activePaths--
		// If no active paths remain, close foundPaths and message channels
		if *activePaths == 0 {
			close(foundPaths)
			close(message)
		}
		mu.Unlock()
	}
}

func manager(wg *sync.WaitGroup, mu *sync.Mutex, activePaths *int, paths chan<- PathInfo, foundPaths <-chan PathInfo, initialPath PathInfo, usageMu *sync.Mutex, maxPathsUsage *int) {
	defer wg.Done()
	mu.Lock()
	*activePaths++ // Increment active paths for the initial path
	mu.Unlock()
	paths <- initialPath

	// Track maximum paths buffer usage
	usageMu.Lock()
	if len(paths) > *maxPathsUsage {
		*maxPathsUsage = len(paths)
	}
	usageMu.Unlock()

	for newPath := range foundPaths {
		paths <- newPath

		// Track maximum paths buffer usage
		usageMu.Lock()
		if len(paths) > *maxPathsUsage {
			*maxPathsUsage = len(paths)
		}
		usageMu.Unlock()
	}
	close(paths) // Close paths when no more new paths to send
}
