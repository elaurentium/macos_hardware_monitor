import SwiftUI

@main
struct MonitorWidgetApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        Settings {
            EmptyView() // Ou pode colocar uma janela de configurações se quiser
        }
    }
}