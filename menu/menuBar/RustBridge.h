#ifndef RustBridge_h
#define RustBridge_h

extern char* get_hardware_stats_json(void);
extern void free_hardware_stats_json(char* ptr);

#endif