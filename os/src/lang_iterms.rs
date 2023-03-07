

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // 实现core中的panic函数，保证程序可以正常运行。
}

