# sleep-on-lan

Simple UDP service to put the computer to sleep remotely.

It accepts the same magic packet as the Wake-On-Lan, that is, a 102-byte packet of 6 * 0xff and 16 * MAC address.

The difference is that to trigger this service, the MAC address must be sent in reverse order, which allow us to use the existing tooling for Wake-On-Lan. For the sake of simplicity, the MAC address that it expects is received as an argument, so it could be anything that resembles a MAC address, actually.

The idea of using the reversed MAC address came from [here](https://github.com/SR-G/sleep-on-lan). If you need something like this I suggest you look into that project, since this one is just a quick toy project for me to play around with rust and use at home.
