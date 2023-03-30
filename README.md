# SinglezoneRGB-ControlCenter
Control your single-zone tongfang-like rgb keyboard on Linux

## ATTENTION
This is a highly experimental POC that uses ACPI to control the keyboard. I only tested it in my personal Laptop (Eluktronics RP-17) and I do not
guarantee that it will work on yours.  
Using it on your device can **be dangerous** and can **brick your keyboard** so be aware of it.
Since I do not have access to their drivers, I am not sure if I'm using the correct header for the APCI calls, but for now it works better than the
control center the brand distributes.

## Contributing

It would be cool if we could have an interface and englobe all the other functionalities, such as turbo mode, "turbo key" assignment, etc. Open an MR and
if you're not sure on where to start, let me know.  

## Requirements
I only tested it on Arch Linux but I imagine any distro with the `acpi_call` kernel module should work.  
Just make sure you have the module loaded using `lsmod | grep acpi_call` (you should see the module name) and that's it.

Cya
