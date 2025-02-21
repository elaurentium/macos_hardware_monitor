import Cocoa

@NSApplicationMain
class AppDelegate: NSObject, NSApplicationDelegate {
    var statusBarItem: NSStatusItem!
    var timer: Timer?

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        let statusBar = NSStatusBar.system
        statusBarItem = statusBar.statusItem(withLength: NSStatusItem.variableLength)
        statusBarItem.button?.title = "Hardware"

        let menu = NSMenu()
        statusBarItem.menu = menu

        timer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: true) { [weak self] _ in
            self?.updateMenu(menu)
        }
        updateMenu(menu)
    }

    func updateMenu(_ menu: NSMenu) {
        menu.removeAllItems()

        let jsonPtr = get_hardware_stats_json()
        guard let jsonCString = jsonPtr else { return }
        let jsonString = String(cString: jsonCString)
        free_hardware_stats_json(jsonCString)

        if let data = jsonString.data(using: .utf8),
           let stats = try? JSONDecoder().decode(HardwareStats.self, from: data) {
            menu.addItem(NSMenuItem(title: "CPU: \(String(format: "%.1f", stats.cpuUsage))%", action: nil, keyEquivalent: ""))
            menu.addItem(NSMenuItem(title: "Mem: \(stats.memoryUsage) / \(stats.memoryTotal) MB", action: nil, keyEquivalent: ""))
            menu.addItem(NSMenuItem(title: "Net In: \(stats.networkIn / 1024) KB", action: nil, keyEquivalent: ""))
            menu.addItem(NSMenuItem(title: "Net Out: \(stats.networkOut / 1024) KB", action: nil, keyEquivalent: ""))
            menu.addItem(NSMenuItem(title: "Disk: \(String(format: "%.1f", stats.diskUsage)) / \(String(format: "%.1f", stats.diskTotal)) GB", action: nil, keyEquivalent: ""))
        } else {
            menu.addItem(NSMenuItem(title: "Erro ao carregar dados", action: nil, keyEquivalent: ""))
        }

        menu.addItem(NSMenuItem.separator())
        menu.addItem(NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q"))
    }
}

struct HardwareStats: Codable {
    let cpuUsage: Float
    let memoryUsage: UInt64
    let memoryTotal: UInt64
    let networkIn: UInt64
    let networkOut: UInt64
    let diskUsage: Double
    let diskTotal: Double

    enum CodingKeys: String, CodingKey {
        case cpuUsage = "cpu_usage"
        case memoryUsage = "memory_usage"
        case memoryTotal = "memory_total"
        case networkIn = "network_in"
        case networkOut = "network_out"
        case diskUsage = "disk_usage"
        case diskTotal = "disk_total"
    }
}