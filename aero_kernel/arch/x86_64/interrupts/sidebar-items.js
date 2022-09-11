window.SIDEBAR_ITEMS = {"constant":[["ICW1_ICW4",""],["ICW1_INIT",""],["ICW1_READ_ISR",""],["ICW4_8086",""],["PIC1_COMMAND",""],["PIC1_DATA",""],["PIC2_COMMAND",""],["PIC2_DATA",""],["PIC_EOI",""]],"fn":[["allocate_vector",""],["disable_interrupts","Wrapper function to the `cli` assembly instruction used to disable interrupts."],["enable_interrupts","Wrapper function to the `sti` assembly instruction used to enable interrupts."],["generic_interrupt_handler",""],["halt","Wrapper function to the `hlt` assembly instruction used to halt the CPU."],["is_enabled","Returns true if interrupts are enabled."],["register_handler","Panics"]],"mod":[["exceptions",""],["idt","The IDT is similar to the Global Descriptor Table in structure."]],"struct":[["APIC_CONTROLLER",""],["ApicController","APIC (Advanced Programmable Interrupt Controller) is an upgraded, advanced version of the PIC chip. It is used for interrupt redirection, and sending interrupts between processors."],["INTERRUPT_CONTROLLER","The global interrupt controller for x86 protected by a read-write lock."],["InterruptController","The interrupt controller interface. The task of an interrupt controller is to end the interrupt, mask the interrupt, send ipi, etc…"],["PIC_CONTROLLER",""],["PicController","PIC (Programmable Interrupt Controller) manages hardware interrupts and sends them to the appropriate system interrupt for the x86 architecture. Since APIC has replaced PIC on modern systems, Aero disables PIC when APIC is avaliable."]]};