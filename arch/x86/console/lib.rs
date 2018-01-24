#![no_std]
use core::fmt;

pub struct Console<'a>{
    slice:&'a mut[u16],
    x:isize,
    y:isize
}
impl<'a> Console<'a>{
    pub fn new(slice:&'a mut [u16])->Console<'a>{
        Console{slice:slice,x:0,y:0}
    }
    fn move_up(&mut self){
        for y in 0..1920{
            self.slice[y]=self.slice[y+80];
        }
        for y in 0..79{
            self.slice[1920+y]=0x0700;
        }
    }

    fn next_line(&mut self){
        self.x=0;
        if self.y==24{
            self.move_up();
        }else{
            self.y+=1;
        }
    }

    pub fn clean(&mut self){
        for y in 0..(self.slice.len()){
            self.slice[y]=0x0700;
        } 
    }
}

impl<'a> fmt::Write for Console<'a>{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let output=s.as_bytes();
        for pos in 0..output.len(){
            if output[pos]==0x0a{
                self.next_line();
            }else{
                let byte=output[pos] as u16;
                self.slice[(self.y*80+self.x) as usize]=7*256 +byte;
                if self.x==79{
                    self.next_line();
                }else{
                    self.x+=1;
                }
            }
        }
        Ok(())
    }
}