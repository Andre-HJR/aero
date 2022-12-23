window.SIDEBAR_ITEMS = {"constant":[["BOOT_GDT_ENTRY_COUNT",""],["GDT_ENTRY_COUNT",""]],"enum":[["Ring",""]],"fn":[["get_kpcr",""],["get_task_state_segment",""],["init","Initialize the actual GDT stored in TLS."],["init_boot","Initialize the bootstrap GDT which is required to initialize TLS (Thread Local Storage) support so, after the kernel heap we will map the TLS section and initialize the actual GDT and then each CPU will have it’s own GDT but we only will have to define it once as a `#[thread_local]`."],["load_cs",""],["load_ds",""],["load_es",""],["load_fs",""],["load_gdt",""],["load_gs",""],["load_ss",""],["load_tss",""]],"static":[["BOOT_GDT",""],["GDT",""],["STK",""]],"struct":[["GdtAccessFlags",""],["GdtDescriptor",""],["GdtEntry",""],["GdtEntryFlags",""],["GdtEntryType",""],["Kpcr",""],["SegmentSelector","Specifies which element to load into a segment from descriptor tables (i.e., is a index to LDT or GDT table with some additional flags)."],["Tss","Although hardware task-switching is not supported in 64-bit mode, a task state segment is required since the it holds information important to 64-bit mode and its not directly related to task switching mechanism."]],"trait":[["__BitFlags",""]]};