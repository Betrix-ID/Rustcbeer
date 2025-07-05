#!/system/bin/sh
# Checking ID shell
if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
   exit 1
fi
#Chking cpu.abi
     if [ ! -f /sdcard/RustcBeer/live/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
             cp /sdcard/RustcBeer/live/target/release/arm64 /sdcard/RustcBeer/server
         fi
     fi
#smart notifications
shell() {
    sor="$1"
    cmd notification post -S bigtext -t '♨️ Custem webui Rustcbeer' 'Tag' "$sor" > /dev/null 2>&1
}
	
# Style display Terminal
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "       ~ Description. RustcBeer Api...... "
    echo
    echo "       - Author                 :  @UnixeID"
    echo "       - Point                    :  - "
    echo "       - Release               :  05 - Jul - 2025"
    echo "       - Name Shell         :  RustcBeer Api"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "    Custem webui Rustcbeer. "
    echo
    sleep 2
     rm -rf /data/local/tmp/server
     cp /sdcard/RustcBeer/server /data/local/tmp
     chmod +x /data/local/tmp/server
     /data/local/tmp/server
set +x