# White noise

Based on an example from https://github.com/jamwaffles/ssd1306

Ported to STM32L031K6T6 (Nucleo32).

Send random raw data to the display, emulating an old untuned TV.

Uses SmallRng as random number generator. 

Note: these are pseudorandom numbers, not suitable for cryptographic or similar purposes.

Known issues: 
* will not work with MSI Range4 and below 
* needs MSI Range6 or higher frequency for 400kHz I2C frequency 