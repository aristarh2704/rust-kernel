pub fn print(){
    static mut X:isize=0;
    let hello="Hello World!";
    unsafe{
        let buffer=0xb8000 as *mut u8;
        let mut pos=X;
        let mut y=0;
        while y<(hello.len() as isize){
            *buffer.offset(pos*2)=*hello.as_ptr().offset(y);
            y=y+1;
            pos+=1;
            if pos>(80 * 25 * 2){
                pos=0;
            }
        }
        X=pos;
    }
}
pub fn clean(x:isize){
    unsafe{
        let mut buffer=0xb8000 as *mut u8;
        let mut y=0;
        while y<(80 * 25 * 2){
            *buffer.offset(y)=0x00;
            y=y+x;
            *buffer.offset(y)=0x07;
            y=y+x;
        }
    } 
}
