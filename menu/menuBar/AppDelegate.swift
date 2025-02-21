import Cocoa
import SwiftUI

class AppDelegate: NSObject, NSApplicationDelegate {
    var statusItem: NSStatusItem?

    func applicationDidFinishLaunching(_ notification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        if let button = statusItem?.button {
            button.image = NSImage(systemSymbolName: "chart.bar", accessibilityDescription: "Monitoramento")
            button.action = #selector(showMenu)
        }
    }

    @objc func showMenu() {
        let menu = NSMenu()

        menu.addItem(NSMenuItem(title: "Abrir Dashboard", action: #selector(openDashboard), keyEquivalent: "D"))
        menu.addItem(NSMenuItem.separator())
        menu.addItem(NSMenuItem(title: "Sair", action: #selector(quitApp), keyEquivalent: "Q"))

        statusItem?.menu = menu
        statusItem?.button?.performClick(nil)
    }

    @objc func openDashboard() {
        print("Abrir Dashboard")
        // TODO: Aqui você pode abrir uma janela, um link ou qualquer ação do app
    }

    @objc func quitApp() {
        NSApplication.shared.terminate(self)
    }
}
