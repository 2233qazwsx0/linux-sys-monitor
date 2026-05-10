#ifndef INSTALL_H
#define INSTALL_H

#include <windows.h>

class InstallerWindow;

bool CheckRustInstalled(wchar_t* versionBuffer, int bufferSize);
bool CheckGitInstalled(wchar_t* versionBuffer, int bufferSize);
bool InstallRust();
bool InstallGit();
bool CloneRepository(const wchar_t* destPath, InstallerWindow* pWindow);
bool BuildProject(const wchar_t* repoPath, InstallerWindow* pWindow);
bool CopyBinary(const wchar_t* repoPath, const wchar_t* installPath, InstallerWindow* pWindow);
bool CreateStartMenuShortcut(const wchar_t* exePath, const wchar_t* installPath);
bool CreateUninstallBatch(const wchar_t* installPath);
bool AddToAutoStart(const wchar_t* exePath);

void RunInstallation(InstallerWindow* pWindow);

#endif
