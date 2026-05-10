#include "window.h"
#include "resource.h"
#include "install.h"
#include <windows.h>
#include <commctrl.h>

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
    LPSTR lpCmdLine, int nCmdShow)
{
    INITCOMMONCONTROLSEX icex;
    icex.dwSize = sizeof(INITCOMMONCONTROLSEX);
    icex.dwICC = ICC_PROGRESS_CLASS | ICC_LISTVIEW_CLASSES | ICC_STANDARD_CLASSES;
    InitCommonControlsEx(&icex);

    CoInitializeEx(NULL, COINIT_APARTMENTTHREADED);

    InstallerWindow window;
    window.Create();

    MSG msg;
    while (GetMessageW(&msg, NULL, 0, 0))
    {
        if (msg.message == WM_KEYDOWN && msg.wParam == VK_ESCAPE)
            continue;

        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    CoUninitialize();

    return (int)msg.wParam;
}
