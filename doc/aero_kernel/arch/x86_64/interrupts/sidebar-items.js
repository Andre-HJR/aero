initSidebarItems({"fn":[["disable_interrupts","Wrapper function to the `cli` assembly instruction used to disable interrupts."],["enable_interrupts","Wrapper function to the `sti` assembly instruction used to enable interrupts."],["halt","Wrapper function to the `hlt` assembly instruction used to halt the CPU."],["pause","Wrapper function to the `pause` assembly instruction used to pause the cpu."]],"macro":[["interrupt","Helper macro that generates an interrupt handler that does not require the interrupt stack as an argument."],["interrupt_error_stack","Helper macro to generate an interrupt exception handler that expects an error code."],["interrupt_stack","Helper macro to generate an interrupt handler that takes the interrupt stack as an argument."]],"mod":[["exceptions",""],["idt","The IDT is similar to the Global Descriptor Table in structure."],["ipi",""],["irq",""]],"struct":[["APIC_CONTROLLER",""],["ApicController","APIC (Advanced Programmable Interrupt Controller) is an upgraded, advanced version of the PIC chip. It is used for interrupt redirection, and sending interrupts between processors."],["INTERRUPT_CONTROLLER","The global interrupt controller for x86 protected by a read-write lock."],["InterruptController","The interrupt controller interface. The task of an interrupt controller is to end the interrupt, mask the interrupt, send ipi, etc…"],["PIC_CONTROLLER",""],["PicController","PIC (Programmable Interrupt Controller) manages hardware interrupts and sends them to the appropriate system interrupt for the x86 architecture. Since APIC has replaced PIC on modern systems, Aero disables PIC when APIC is avaliable."]]});