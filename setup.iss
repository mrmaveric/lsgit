#define MyAppName "LSGit"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "James Bull"
#define MyAppExeName "lsgit.exe"

[Setup]
AppId={{D6EFE474-A7BE-4E6A-B341-78E04989E143}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName=C:\tool
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
OutputBaseFilename=lsgit-setup
Compression=lzma
SolidCompression=yes
PrivilegesRequired=admin
ArchitecturesAllowed=x64compatible arm64
ArchitecturesInstallIn64BitMode=x64compatible arm64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "dist\lsgit_windows_amd64_v1\lsgit.exe"; DestDir: "{app}"; Flags: ignoreversion; Check: IsAMD64
Source: "dist\lsgit_windows_arm64_v8.0\lsgit.exe"; DestDir: "{app}"; Flags: ignoreversion; Check: IsARM64

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: NeedsAddPath(ExpandConstant('{app}'))

[Code]
function IsAMD64: Boolean;
begin
  Result := ProcessorArchitecture = paX64;
end;

function IsARM64: Boolean;
begin
  Result := ProcessorArchitecture = paARM64;
end;

function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Path: string;
  AppPath: string;
  P: Integer;
begin
  if CurUninstallStep = usUninstall then
  begin
    if RegQueryStringValue(HKEY_LOCAL_MACHINE,
      'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
      'Path', Path) then
    begin
      AppPath := ExpandConstant('{app}');
      P := Pos(';' + Uppercase(AppPath) + ';', ';' + Uppercase(Path) + ';');
      if P = 0 then
        P := Pos(';' + Uppercase(AppPath), ';' + Uppercase(Path));
      if P = 0 then
        P := Pos(Uppercase(AppPath) + ';', Uppercase(Path));
      if P = 0 then
        P := Pos(Uppercase(AppPath), Uppercase(Path));
      if P > 0 then
      begin
        Delete(Path, P - 1, Length(AppPath) + 1);
        RegWriteStringValue(HKEY_LOCAL_MACHINE,
          'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
          'Path', Path);
      end;
    end;
  end;
end;

