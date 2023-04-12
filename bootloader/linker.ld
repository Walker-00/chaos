ENTRY(start)

SECTIONS
{
	. = 1M;

	.boot :
	{
		KEEP(*(.multiboot_header_func))
	}

	.text :
	{
		*(.text)
	}
}
