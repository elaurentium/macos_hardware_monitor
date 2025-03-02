import Cocoa
import RustBridge

struct HardwareInfo: Codable {
    let cpuUsage: UInt32
    let memoryUsed: Float
    let memoryTotal: Float
    let networkIn: Float
    let networkOut: Float
    let diskUsed: Double
    let diskTotal: Double
    
    enum CodingKeys: String, CodingKey {
        case cpuUsage = "cpu_usage"
        case memoryUsed = "memory_used"
        case memoryTotal = "memory_total"
        case networkIn = "network_in"
        case networkOut = "network_out"
        case diskUsed = "disk_used"
        case diskTotal = "disk_total"
    }
}

func updateUI(with jsonString: String) {
    //TODO
    }
}
