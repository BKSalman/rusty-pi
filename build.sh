echo "removing target folder"
rm -rf target/
echo "building project"
cargo rustc --bin rusty-pi -- -C link-arg=--script=./linker.ld
echo "copying to 'kernel7.img'"
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rusty-pi ./kernel7.img
echo "copying 'kernel7.img' to sd card"
cp kernel7.img /run/media/salman/BOOT/
