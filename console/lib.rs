#![no_std]

fn move_up(x:isize){
    let buffer=0xb8000 as *mut u16;
    for y in 0..x{
        unsafe{
            let p=buffer.offset(y);
            let p1=*buffer.offset(y+80);
            *p=p1
        };
    }
    for y in 0..79{
        unsafe{
            *buffer.offset(1920+y)=0x0700;
        }
    }
}

fn next_line(x:&mut isize,y:&mut isize){
    *x=0;
    if *y==24{
        move_up(1920);
    }else{
        *y+=1;
    }
}

pub fn print(buffer:&mut[u16],output:&[u8]){
    static mut X:isize=0;
    static mut Y:isize=0;
    for pos in 0..output.len(){
        unsafe{
            if output[pos]==0x0a{
                next_line(&mut X,&mut Y);
            }else{
                let byte=output[pos] as u16;
                buffer[(Y*80+X) as usize]=7*256 +byte;
                if X==79{
                    next_line(&mut X,&mut Y);
                }else{
                    X+=1;
                }
            }
        }
    }
}

pub fn clean(buffer:&mut[u16]){
    for y in 0..(buffer.len()){
        buffer[y]=0x0700;
    } 
}
 pub fn print_number(buffer:&mut[u16],mut x:u16){
     let mut arr=[0u8;11];
     for pos in 0..arr.len()-1{
        arr[9-pos]=(x%10+48) as u8;
        x=x/10;
     }
     arr[10]=0x0a;
     print(buffer,&arr);
 }