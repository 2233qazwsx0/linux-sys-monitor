#include "window.h"
#include "resource.h"
#include <windowsx.h>
#include <commctrl.h>
#include <shlobj.h>

#pragma comment(lib, "comctl32.lib")

static const wchar_t g_szClassName[] = L"LinuxSysMonitorInstaller";

static wchar_t g_szTitle[] = L"Linux 系统监控 - 安装向导";
static wchar_t g_szWelcome[] = L"欢迎安装 Linux 系统监控\n\n此程序将帮助您在 Windows 系统上安装 Linux 系统监控工具。\n\n点击\"下一步\"继续安装。";
static wchar_t g_szSubtitleWelcome[] = L"Linux 系统监控 安装向导";
static wchar_t g_szSubtitleDeps[] = L"检查依赖项";
static wchar_t g_szSubtitlePath[] = L"选择安装位置";
static wchar_t g_szSubtitleInstalling[] = L"正在安装...";
static wchar_t g_szSubtitleFinish[] = L"安装完成";

static wchar_t g_szNext[] = L"下一步(&N) >";
static wchar_t g_szBack[] = L"< 上一步(&B)";
static wchar_t g_szCancel[] = L"取消";
static wchar_t g_szFinish[] = L"完成(&F)";
static wchar_t g_szBrowse[] = L"浏览...";

static wchar_t g_szDepRustLabel[] = L"Rust 编译器:";
static wchar_t g_szDepGitLabel[] = L"Git 版本控制:";
static wchar_t g_szDepStatusUnknown[] = L"正在检查...";
static wchar_t g_szDepStatusNotFound[] = L"未找到";
static wchar_t g_szDepStatusFound[] = L"已安装";
static wchar_t g_szDepStatusInstalling[] = L"正在安装...";
static wchar_t g_szInstallRust[] = L"安装 Rust";
static wchar_t g_szInstallGit[] = L"下载 Git";

static wchar_t g_szPathLabel[] = L"选择安装目录:";
static wchar_t g_szDefaultPath[MAX_PATH] = L"C:\\Program Files\\LinuxSysMonitor";

static wchar_t g_szInstallStatus[] = L"正在准备安装...";
static wchar_t g_szLaunchLabel[] = L"立即运行 Linux 系统监控(&R)";

static wchar_t g_szFinishTitle[] = L"已成功安装 Linux 系统监控";
static wchar_t g_szFinishText[] = L"点击\"完成\"结束安装。\n\n您可以通过开始菜单启动程序。";

static HFONT g_hTitleFont = NULL;
static HFONT g_hNormalFont = NULL;
static HBRUSH g_hBgBrush = NULL;
static HBRUSH g_hWhiteBrush = NULL;

static LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam);

InstallerWindow::InstallerWindow()
    : m_hwnd(NULL)
    , m_hInstance(NULL)
    , m_currentPage(PAGE_WELCOME)
    , m_autoLaunch(false)
    , m_hTitle(NULL)
    , m_hSubtitle(NULL)
    , m_hWelcomeText(NULL)
    , m_hStatusText(NULL)
    , m_hProgressBar(NULL)
    , m_hLogText(NULL)
    , m_hPathEdit(NULL)
    , m_hBrowseBtn(NULL)
    , m_hNextBtn(NULL)
    , m_hBackBtn(NULL)
    , m_hCancelBtn(NULL)
    , m_hFinishBtn(NULL)
    , m_hLaunchCheckbox(NULL)
    , m_hRustStatus(NULL)
    , m_hGitStatus(NULL)
    , m_hInstallRustBtn(NULL)
    , m_hInstallGitBtn(NULL)
    , m_installationStarted(false)
{
    m_rustInfo.status = DEP_UNKNOWN;
    m_gitInfo.status = DEP_UNKNOWN;
}

InstallerWindow::~InstallerWindow()
{
    if (g_hTitleFont) DeleteObject(g_hTitleFont);
    if (g_hNormalFont) DeleteObject(g_hNormalFont);
    if (g_hBgBrush) DeleteObject(g_hBgBrush);
    if (g_hWhiteBrush) DeleteObject(g_hWhiteBrush);
}

bool InstallerWindow::Create()
{
    WNDCLASSEXW wcex;
    memset(&wcex, 0, sizeof(wcex));
    wcex.cbSize = sizeof(WNDCLASSEXW);
    wcex.style = CS_HREDRAW | CS_VREDRAW;
    wcex.lpfnWndProc = WndProc;
    wcex.hInstance = m_hInstance;
    wcex.hIcon = LoadIconW(m_hInstance, MAKEINTRESOURCEW(IDI_APP));
    wcex.hCursor = LoadCursorW(NULL, (LPCWSTR)IDC_ARROW);
    wcex.hbrBackground = CreateSolidBrush(RGB(240, 240, 240));
    wcex.lpszClassName = g_szClassName;

    if (!RegisterClassExW(&wcex))
    {
        return false;
    }

    m_hwnd = CreateWindowExW(
        WS_EX_COMPOSITED,
        g_szClassName,
        g_szTitle,
        WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
        CW_USEDEFAULT, CW_USEDEFAULT,
        520, 400,
        NULL, NULL,
        m_hInstance,
        this
    );

    if (!m_hwnd)
    {
        return false;
    }

    NONCLIENTMETRICSW ncm;
    memset(&ncm, 0, sizeof(ncm));
    ncm.cbSize = sizeof(NONCLIENTMETRICSW);
    if (SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, sizeof(ncm), &ncm, 0))
    {
        LOGFONTW lfTitle;
        memcpy(&lfTitle, &ncm.lfMessageFont, sizeof(LOGFONTW));
        lfTitle.lfWeight = FW_BOLD;
        lfTitle.lfHeight = -24;
        g_hTitleFont = CreateFontIndirectW(&lfTitle);

        g_hNormalFont = CreateFontIndirectW(&ncm.lfMessageFont);
    }

    if (!g_hNormalFont)
    {
        g_hNormalFont = CreateFontW(14, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
            DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE, L"Microsoft YaHei UI");
    }

    g_hBgBrush = CreateSolidBrush(RGB(240, 240, 240));
    g_hWhiteBrush = CreateSolidBrush(RGB(255, 255, 255));

    CreateControls();

    SetWindowLongPtrW(m_hwnd, GWLP_USERDATA, (LONG_PTR)this);

    return true;
}

void InstallerWindow::Show(int nCmdShow)
{
    ShowWindow(m_hwnd, nCmdShow);
    UpdateWindow(m_hwnd);
}

void InstallerWindow::CreateControls()
{
    RECT rcClient;
    GetClientRect(m_hwnd, &rcClient);
    int clientWidth = rcClient.right - rcClient.left;
    int clientHeight = rcClient.bottom - rcClient.top;

    int btnWidth = 90;
    int btnHeight = 28;
    int btnSpacing = 10;
    int bottomY = clientHeight - btnHeight - 20;

    m_hTitle = CreateWindowExW(0, L"STATIC", g_szTitle,
        WS_CHILD | SS_LEFT | SS_ENDELLIPSIS,
        20, 15, clientWidth - 40, 35,
        m_hwnd, (HMENU)(INT_PTR)IDC_TITLE, m_hInstance, NULL);

    m_hSubtitle = CreateWindowExW(0, L"STATIC", g_szSubtitleWelcome,
        WS_CHILD | SS_LEFT,
        20, 50, clientWidth - 40, 25,
        m_hwnd, (HMENU)(INT_PTR)IDC_SUBTITLE, m_hInstance, NULL);

    m_hWelcomeText = CreateWindowExW(0, L"STATIC", g_szWelcome,
        WS_CHILD | SS_LEFT | SS_NOPREFIX,
        20, 90, clientWidth - 40, 150,
        m_hwnd, (HMENU)(INT_PTR)IDC_WELCOME_TEXT, m_hInstance, NULL);

    m_hStatusText = CreateWindowExW(WS_EX_TRANSPARENT, L"STATIC", g_szInstallStatus,
        WS_CHILD | SS_LEFT,
        20, 85, clientWidth - 40, 20,
        m_hwnd, (HMENU)(INT_PTR)IDC_STATUS_TEXT, m_hInstance, NULL);

    m_hProgressBar = CreateWindowExW(0, L"msctls_progress32", NULL,
        WS_CHILD | PBS_SMOOTH,
        20, 110, clientWidth - 40, 25,
        m_hwnd, (HMENU)(INT_PTR)IDC_PROGRESS_BAR, m_hInstance, NULL);

    m_hLogText = CreateWindowExW(WS_EX_CLIENTEDGE, L"EDIT", NULL,
        WS_CHILD | ES_LEFT | ES_MULTILINE | ES_READONLY | ES_AUTOVSCROLL | WS_VSCROLL,
        20, 145, clientWidth - 40, 120,
        m_hwnd, (HMENU)(INT_PTR)IDC_LOG_TEXT, m_hInstance, NULL);

    m_hRustStatus = CreateWindowExW(0, L"STATIC", g_szDepStatusUnknown,
        WS_CHILD | SS_LEFT,
        120, 100, 200, 20,
        m_hwnd, (HMENU)(INT_PTR)IDC_DEP_RUST, m_hInstance, NULL);

    m_hGitStatus = CreateWindowExW(0, L"STATIC", g_szDepStatusUnknown,
        WS_CHILD | SS_LEFT,
        120, 130, 200, 20,
        m_hwnd, (HMENU)(INT_PTR)IDC_DEP_GIT, m_hInstance, NULL);

    m_hInstallRustBtn = CreateWindowExW(0, L"BUTTON", g_szInstallRust,
        WS_CHILD | BS_PUSHBUTTON,
        330, 96, 100, 26,
        m_hwnd, (HMENU)(INT_PTR)IDC_INSTALL_RUST_BTN, m_hInstance, NULL);

    m_hInstallGitBtn = CreateWindowExW(0, L"BUTTON", g_szInstallGit,
        WS_CHILD | BS_PUSHBUTTON,
        330, 126, 100, 26,
        m_hwnd, (HMENU)(INT_PTR)IDC_INSTALL_GIT_BTN, m_hInstance, NULL);

    m_hPathEdit = CreateWindowExW(WS_EX_CLIENTEDGE, L"EDIT", g_szDefaultPath,
        WS_CHILD | ES_LEFT | ES_AUTOHSCROLL,
        120, 100, clientWidth - 250, 24,
        m_hwnd, (HMENU)(INT_PTR)IDC_PATH_EDIT, m_hInstance, NULL);

    m_hBrowseBtn = CreateWindowExW(0, L"BUTTON", g_szBrowse,
        WS_CHILD | BS_PUSHBUTTON,
        clientWidth - 115, 96, 95, 28,
        m_hwnd, (HMENU)(INT_PTR)IDC_BROWSE_BTN, m_hInstance, NULL);

    m_hLaunchCheckbox = CreateWindowExW(0, L"BUTTON", g_szLaunchLabel,
        WS_CHILD | BS_AUTOCHECKBOX,
        20, 130, clientWidth - 40, 25,
        m_hwnd, (HMENU)(INT_PTR)IDC_LAUNCH_CHECKBOX, m_hInstance, NULL);

    int cancelX = clientWidth - btnWidth - 20;
    int backX = cancelX - btnSpacing - btnWidth;
    int nextX = backX - btnSpacing - btnWidth;

    m_hCancelBtn = CreateWindowExW(0, L"BUTTON", g_szCancel,
        WS_CHILD | BS_PUSHBUTTON,
        cancelX, bottomY, btnWidth, btnHeight,
        m_hwnd, (HMENU)(INT_PTR)IDC_CANCEL_BTN, m_hInstance, NULL);

    m_hBackBtn = CreateWindowExW(0, L"BUTTON", g_szBack,
        WS_CHILD | BS_PUSHBUTTON,
        backX, bottomY, btnWidth, btnHeight,
        m_hwnd, (HMENU)(INT_PTR)IDC_BACK_BTN, m_hInstance, NULL);

    m_hNextBtn = CreateWindowExW(0, L"BUTTON", g_szNext,
        WS_CHILD | BS_PUSHBUTTON,
        nextX, bottomY, btnWidth, btnHeight,
        m_hwnd, (HMENU)(INT_PTR)IDC_NEXT_BTN, m_hInstance, NULL);

    m_hFinishBtn = CreateWindowExW(0, L"BUTTON", g_szFinish,
        WS_CHILD | BS_PUSHBUTTON,
        nextX, bottomY, btnWidth, btnHeight,
        m_hwnd, (HMENU)(INT_PTR)IDC_FINISH_BTN, m_hInstance, NULL);

    SendMessageW(m_hProgressBar, PBM_SETRANGE, 0, MAKELPARAM(0, 100));
    SendMessageW(m_hProgressBar, PBM_SETPOS, 0, 0);

    SendMessageW(m_hTitle, WM_SETFONT, (WPARAM)(g_hTitleFont ? g_hTitleFont : g_hNormalFont), TRUE);
    SendMessageW(m_hSubtitle, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hWelcomeText, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hStatusText, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hLogText, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hRustStatus, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hGitStatus, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hPathEdit, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hCancelBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hBackBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hNextBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hFinishBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hLaunchCheckbox, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hBrowseBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hInstallRustBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    SendMessageW(m_hInstallGitBtn, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);

    ShowPage(PAGE_WELCOME);
}

void InstallerWindow::ShowPage(InstallerPage page)
{
    HideAllPages();
    m_currentPage = page;

    switch (page)
    {
    case PAGE_WELCOME:
        ShowWelcomePage();
        break;
    case PAGE_DEPENDENCIES:
        ShowDependenciesPage();
        break;
    case PAGE_INSTALL_PATH:
        ShowPathPage();
        break;
    case PAGE_INSTALLING:
        ShowInstallingPage();
        break;
    case PAGE_FINISH:
        ShowFinishPage();
        break;
    }
}

void InstallerWindow::HideAllPages()
{
    ShowWindow(m_hTitle, SW_HIDE);
    ShowWindow(m_hSubtitle, SW_HIDE);
    ShowWindow(m_hWelcomeText, SW_HIDE);
    ShowWindow(m_hStatusText, SW_HIDE);
    ShowWindow(m_hProgressBar, SW_HIDE);
    ShowWindow(m_hLogText, SW_HIDE);
    ShowWindow(m_hPathEdit, SW_HIDE);
    ShowWindow(m_hBrowseBtn, SW_HIDE);
    ShowWindow(m_hNextBtn, SW_HIDE);
    ShowWindow(m_hBackBtn, SW_HIDE);
    ShowWindow(m_hCancelBtn, SW_HIDE);
    ShowWindow(m_hFinishBtn, SW_HIDE);
    ShowWindow(m_hLaunchCheckbox, SW_HIDE);
    ShowWindow(m_hRustStatus, SW_HIDE);
    ShowWindow(m_hGitStatus, SW_HIDE);
    ShowWindow(m_hInstallRustBtn, SW_HIDE);
    ShowWindow(m_hInstallGitBtn, SW_HIDE);
}

void InstallerWindow::ShowWelcomePage()
{
    SetWindowTextW(m_hSubtitle, g_szSubtitleWelcome);

    ShowWindow(m_hTitle, SW_SHOW);
    ShowWindow(m_hSubtitle, SW_SHOW);
    ShowWindow(m_hWelcomeText, SW_SHOW);
    ShowWindow(m_hNextBtn, SW_SHOW);
    ShowWindow(m_hCancelBtn, SW_SHOW);

    Button_Enable(m_hNextBtn, TRUE);
    Button_Enable(m_hCancelBtn, TRUE);
}

void InstallerWindow::ShowDependenciesPage()
{
    SetWindowTextW(m_hSubtitle, g_szSubtitleDeps);

    HWND hRustLabel = CreateWindowExW(0, L"STATIC", g_szDepRustLabel,
        WS_CHILD | SS_LEFT,
        20, 100, 100, 20,
        m_hwnd, (HMENU)(INT_PTR)0, m_hInstance, NULL);
    SendMessageW(hRustLabel, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    ShowWindow(hRustLabel, SW_SHOW);

    HWND hGitLabel = CreateWindowExW(0, L"STATIC", g_szDepGitLabel,
        WS_CHILD | SS_LEFT,
        20, 130, 100, 20,
        m_hwnd, (HMENU)(INT_PTR)0, m_hInstance, NULL);
    SendMessageW(hGitLabel, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    ShowWindow(hGitLabel, SW_SHOW);

    ShowWindow(m_hTitle, SW_SHOW);
    ShowWindow(m_hSubtitle, SW_SHOW);
    ShowWindow(m_hRustStatus, SW_SHOW);
    ShowWindow(m_hGitStatus, SW_SHOW);
    ShowWindow(m_hInstallRustBtn, SW_SHOW);
    ShowWindow(m_hInstallGitBtn, SW_SHOW);
    ShowWindow(m_hNextBtn, SW_SHOW);
    ShowWindow(m_hBackBtn, SW_SHOW);
    ShowWindow(m_hCancelBtn, SW_SHOW);

    Button_Enable(m_hBackBtn, TRUE);
    Button_Enable(m_hNextBtn, m_rustInfo.status == DEP_FOUND && m_gitInfo.status == DEP_FOUND);
    Button_Enable(m_hCancelBtn, TRUE);
    Button_Enable(m_hInstallRustBtn, m_rustInfo.status != DEP_FOUND);
    Button_Enable(m_hInstallGitBtn, m_gitInfo.status != DEP_FOUND);
}

void InstallerWindow::ShowPathPage()
{
    SetWindowTextW(m_hSubtitle, g_szSubtitlePath);

    HWND hPathLabel = CreateWindowExW(0, L"STATIC", g_szPathLabel,
        WS_CHILD | SS_LEFT,
        20, 100, 100, 20,
        m_hwnd, (HMENU)(INT_PTR)0, m_hInstance, NULL);
    SendMessageW(hPathLabel, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    ShowWindow(hPathLabel, SW_SHOW);

    ShowWindow(m_hTitle, SW_SHOW);
    ShowWindow(m_hSubtitle, SW_SHOW);
    ShowWindow(m_hPathEdit, SW_SHOW);
    ShowWindow(m_hBrowseBtn, SW_SHOW);
    ShowWindow(m_hNextBtn, SW_SHOW);
    ShowWindow(m_hBackBtn, SW_SHOW);
    ShowWindow(m_hCancelBtn, SW_SHOW);

    if (m_installPath.empty())
    {
        SetWindowTextW(m_hPathEdit, g_szDefaultPath);
    }
    else
    {
        SetWindowTextW(m_hPathEdit, m_installPath.c_str());
    }

    Button_Enable(m_hBackBtn, TRUE);
    Button_Enable(m_hNextBtn, TRUE);
    Button_Enable(m_hCancelBtn, TRUE);
}

void InstallerWindow::ShowInstallingPage()
{
    SetWindowTextW(m_hSubtitle, g_szSubtitleInstalling);

    ShowWindow(m_hTitle, SW_SHOW);
    ShowWindow(m_hSubtitle, SW_SHOW);
    ShowWindow(m_hStatusText, SW_SHOW);
    ShowWindow(m_hProgressBar, SW_SHOW);
    ShowWindow(m_hLogText, SW_SHOW);
    ShowWindow(m_hCancelBtn, SW_SHOW);

    SendMessageW(m_hProgressBar, PBM_SETPOS, 0, 0);
    SetWindowTextW(m_hStatusText, g_szInstallStatus);
    SetWindowTextW(m_hLogText, L"");
}

void InstallerWindow::ShowFinishPage()
{
    SetWindowTextW(m_hSubtitle, g_szSubtitleFinish);

    HWND hFinishTitle = CreateWindowExW(0, L"STATIC", g_szFinishTitle,
        WS_CHILD | SS_LEFT,
        20, 90, 460, 25,
        m_hwnd, (HMENU)(INT_PTR)0, m_hInstance, NULL);
    SendMessageW(hFinishTitle, WM_SETFONT, (WPARAM)(g_hTitleFont ? g_hTitleFont : g_hNormalFont), TRUE);
    ShowWindow(hFinishTitle, SW_SHOW);

    HWND hFinishText = CreateWindowExW(0, L"STATIC", g_szFinishText,
        WS_CHILD | SS_LEFT,
        20, 125, 460, 60,
        m_hwnd, (HMENU)(INT_PTR)0, m_hInstance, NULL);
    SendMessageW(hFinishText, WM_SETFONT, (WPARAM)g_hNormalFont, TRUE);
    ShowWindow(hFinishText, SW_SHOW);

    ShowWindow(m_hTitle, SW_SHOW);
    ShowWindow(m_hSubtitle, SW_SHOW);
    ShowWindow(m_hLaunchCheckbox, SW_SHOW);
    ShowWindow(m_hFinishBtn, SW_SHOW);
    ShowWindow(m_hCancelBtn, SW_SHOW);

    Button_SetCheck(m_hLaunchCheckbox, m_autoLaunch ? BST_CHECKED : BST_UNCHECKED);
}

void InstallerWindow::SetInstallPath(const std::wstring& path)
{
    m_installPath = path;
    if (m_hPathEdit && IsWindowVisible(m_hPathEdit))
    {
        SetWindowTextW(m_hPathEdit, path.c_str());
    }
}

void InstallerWindow::SetProgress(int percent)
{
    SendMessageW(m_hProgressBar, PBM_SETPOS, percent, 0);
}

void InstallerWindow::UpdateLog(const wchar_t* message)
{
    if (!m_hLogText) return;

    int len = GetWindowTextLengthW(m_hLogText);
    wchar_t* existing = new wchar_t[len + 1];
    GetWindowTextW(m_hLogText, existing, len + 1);

    std::wstring log = existing;
    delete[] existing;

    if (!log.empty())
        log += L"\r\n";
    log += message;

    SetWindowTextW(m_hLogText, log.c_str());

    int lineCount = SendMessageW(m_hLogText, EM_GETLINECOUNT, 0, 0);
    SendMessageW(m_hLogText, EM_LINESCROLL, 0, lineCount);
}

void InstallerWindow::SetStatusText(const wchar_t* text)
{
    if (m_hStatusText)
        SetWindowTextW(m_hStatusText, text);
}

void InstallerWindow::EnableControls(bool enable)
{
    Button_Enable(m_hNextBtn, enable);
    Button_Enable(m_hBackBtn, enable);
    Button_Enable(m_hCancelBtn, enable);
}

void InstallerWindow::SetRustStatus(DependencyStatus status, const wchar_t* version)
{
    m_rustInfo.status = status;
    if (version && *version)
        m_rustInfo.version = version;

    if (!m_hRustStatus) return;

    switch (status)
    {
    case DEP_UNKNOWN:
        SetWindowTextW(m_hRustStatus, g_szDepStatusUnknown);
        break;
    case DEP_NOT_FOUND:
        SetWindowTextW(m_hRustStatus, g_szDepStatusNotFound);
        break;
    case DEP_FOUND:
    {
        std::wstring text = g_szDepStatusFound;
        if (!m_rustInfo.version.empty())
        {
            text += L" (";
            text += m_rustInfo.version;
            text += L")";
        }
        SetWindowTextW(m_hRustStatus, text.c_str());
        break;
    }
    case DEP_INSTALLING:
        SetWindowTextW(m_hRustStatus, g_szDepStatusInstalling);
        break;
    }

    if (m_currentPage == PAGE_DEPENDENCIES)
    {
        Button_Enable(m_hInstallRustBtn, status != DEP_FOUND);
        Button_Enable(m_hNextBtn, m_rustInfo.status == DEP_FOUND && m_gitInfo.status == DEP_FOUND);
    }
}

void InstallerWindow::SetGitStatus(DependencyStatus status, const wchar_t* version)
{
    m_gitInfo.status = status;
    if (version && *version)
        m_gitInfo.version = version;

    if (!m_hGitStatus) return;

    switch (status)
    {
    case DEP_UNKNOWN:
        SetWindowTextW(m_hGitStatus, g_szDepStatusUnknown);
        break;
    case DEP_NOT_FOUND:
        SetWindowTextW(m_hGitStatus, g_szDepStatusNotFound);
        break;
    case DEP_FOUND:
    {
        std::wstring text = g_szDepStatusFound;
        if (!m_gitInfo.version.empty())
        {
            text += L" (";
            text += m_gitInfo.version;
            text += L")";
        }
        SetWindowTextW(m_hGitStatus, text.c_str());
        break;
    }
    case DEP_INSTALLING:
        SetWindowTextW(m_hGitStatus, g_szDepStatusInstalling);
        break;
    }

    if (m_currentPage == PAGE_DEPENDENCIES)
    {
        Button_Enable(m_hInstallGitBtn, status != DEP_FOUND);
        Button_Enable(m_hNextBtn, m_rustInfo.status == DEP_FOUND && m_gitInfo.status == DEP_FOUND);
    }
}

void InstallerWindow::PostInstallComplete()
{
    PostMessageW(m_hwnd, MSG_INSTALL_COMPLETE, 0, 0);
}

void InstallerWindow::PostInstallError(const wchar_t* error)
{
    PostMessageW(m_hwnd, MSG_INSTALL_ERROR, 0, (LPARAM)error);
}

void InstallerWindow::PostDepCheckDone()
{
    PostMessageW(m_hwnd, MSG_DEP_CHECK_DONE, 0, 0);
}

InstallerWindow* InstallerWindow::GetFromHwnd(HWND hwnd)
{
    return (InstallerWindow*)GetWindowLongPtrW(hwnd, GWLP_USERDATA);
}

void InstallerWindow::OnNextClicked()
{
    wchar_t pathBuffer[MAX_PATH];
    GetWindowTextW(m_hPathEdit, pathBuffer, MAX_PATH);
    m_installPath = pathBuffer;

    switch (m_currentPage)
    {
    case PAGE_WELCOME:
        ShowPage(PAGE_DEPENDENCIES);
        break;
    case PAGE_DEPENDENCIES:
        ShowPage(PAGE_INSTALL_PATH);
        break;
    case PAGE_INSTALL_PATH:
        ShowPage(PAGE_INSTALLING);
        StartInstallation();
        break;
    }
}

void InstallerWindow::OnBackClicked()
{
    switch (m_currentPage)
    {
    case PAGE_DEPENDENCIES:
        ShowPage(PAGE_WELCOME);
        break;
    case PAGE_INSTALL_PATH:
        ShowPage(PAGE_DEPENDENCIES);
        break;
    }
}

void InstallerWindow::OnCancelClicked()
{
    if (m_currentPage == PAGE_INSTALLING && m_installationStarted)
    {
        if (MessageBoxW(m_hwnd,
            L"安装正在进行中，确定要取消吗？",
            L"取消安装",
            MB_YESNO | MB_ICONQUESTION) != IDYES)
        {
            return;
        }
    }

    EndDialog(m_hwnd, IDCANCEL);
}

void InstallerWindow::OnFinishClicked()
{
    if (Button_GetCheck(m_hLaunchCheckbox) == BST_CHECKED)
    {
        std::wstring exePath = m_installPath + L"\\linux-sys-monitor.exe";
        ShellExecuteW(NULL, L"open", exePath.c_str(), NULL, NULL, SW_SHOWNORMAL);
    }

    EndDialog(m_hwnd, IDOK);
}

void InstallerWindow::OnBrowseClicked()
{
    BROWSEINFOW bi;
    memset(&bi, 0, sizeof(bi));
    bi.hwndOwner = m_hwnd;
    bi.lpszTitle = L"选择安装目录";
    bi.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;

    PIDLIST_ABSOLUTE pidl = SHBrowseForFolderW(&bi);
    if (pidl)
    {
        wchar_t pathBuffer[MAX_PATH];
        if (SHGetPathFromIDListW(pidl, pathBuffer))
        {
            SetWindowTextW(m_hPathEdit, pathBuffer);
            m_installPath = pathBuffer;
        }

        CoTaskMemFree(pidl);
    }
}

void InstallerWindow::OnInstallRustClicked()
{
    SetRustStatus(DEP_INSTALLING);
    ShellExecuteW(NULL, L"open", L"https://win.rustup.rs", NULL, NULL, SW_SHOWNORMAL);
}

void InstallerWindow::OnInstallGitClicked()
{
    SetGitStatus(DEP_INSTALLING);
    ShellExecuteW(NULL, L"open", L"https://git-scm.com/download/win", NULL, NULL, SW_SHOWNORMAL);
}

void InstallerWindow::StartInstallation()
{
    m_installationStarted = true;
    EnableControls(false);

    HANDLE hThread = CreateThread(NULL, 0, InstallationThread, this, 0, NULL);
    if (hThread)
    {
        CloseHandle(hThread);
    }
}

DWORD WINAPI InstallerWindow::InstallationThread(LPVOID lpParam)
{
    InstallerWindow* pWindow = (InstallerWindow*)lpParam;
    if (!pWindow)
        return 1;

    extern void RunInstallation(InstallerWindow* pWindow);
    RunInstallation(pWindow);

    return 0;
}

static LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    InstallerWindow* pWindow = InstallerWindow::GetFromHwnd(hwnd);

    switch (msg)
    {
    case WM_CREATE:
        return 0;

    case WM_COMMAND:
    {
        if (!pWindow) return 0;
        int id = LOWORD(wParam);
        int notify = HIWORD(wParam);

        if (notify == BN_CLICKED)
        {
            switch (id)
            {
            case IDC_NEXT_BTN:
                pWindow->OnNextClicked();
                return 0;
            case IDC_BACK_BTN:
                pWindow->OnBackClicked();
                return 0;
            case IDC_CANCEL_BTN:
                pWindow->OnCancelClicked();
                return 0;
            case IDC_FINISH_BTN:
                pWindow->OnFinishClicked();
                return 0;
            case IDC_BROWSE_BTN:
                pWindow->OnBrowseClicked();
                return 0;
            case IDC_INSTALL_RUST_BTN:
                pWindow->OnInstallRustClicked();
                return 0;
            case IDC_INSTALL_GIT_BTN:
                pWindow->OnInstallGitClicked();
                return 0;
            }
        }
        return 0;
    }

    case WM_CTLCOLORSTATIC:
    {
        HDC hdcStatic = (HDC)wParam;
        SetBkColor(hdcStatic, RGB(240, 240, 240));
        if (g_hBgBrush)
            return (LRESULT)g_hBgBrush;
        return (LRESULT)GetSysColorBrush(COLOR_3DFACE);
    }

    case WM_CTLCOLOREDIT:
    {
        HDC hdc = (HDC)wParam;
        SetBkColor(hdc, RGB(255, 255, 255));
        if (g_hWhiteBrush)
            return (LRESULT)g_hWhiteBrush;
        return (LRESULT)GetSysColorBrush(COLOR_WINDOW);
    }

    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;

    default:
        return DefWindowProcW(hwnd, msg, wParam, lParam);
    }

    return 0;
}
