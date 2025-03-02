import Cocoa

@NSApplicationMain
@objc class AppDelegate: NSObject, NSApplicationDelegate {
    var statusItem: NSStatusItem!

    func applicationDidFinishLaunching(_ notification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        
        if let button = statusItem.button {
            button.title = "Carregando..."
            
            // Chama a função Rust em uma thread separada para não bloquear
            DispatchQueue.global().async {
                let data = self.getRustData()
                DispatchQueue.main.async {
                    button.title = data
                }
            }
        }
    }

    func getRustData() -> String {
        let cString = get_hardware_stats_json();
        let swiftString = String(cString: cString);
        free_hardware_stats_json(cString);
        return swiftString;
    }

}