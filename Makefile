default:
	cargo rumpbake hw_virtio
	genisoimage -r -o stubdata.iso data
	rumprun qemu \
		-I if,vioif,'-net tap,script=no,ifname=tap0' \
		-b stubdata.iso,/etc \
		-W if,inet,static,10.0.23.1/24 \
		-i steveklabnikdotcom.img
