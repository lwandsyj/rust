// 棋子的颜色，白色和黑色
#[derive(Debug,Copy, Clone,PartialEq)]
pub enum PieceColor { // pub 表示对外公开
    White,
    Black,
}

#[derive(Debug,Copy, Clone,PartialEq)]
pub struct GamePiece{
    pub color:PieceColor, // 属性也必须设置pub
    pub crowned:bool
}

// 为结构赋值方法
impl GamePiece{
    pub fn new(color:PieceColor)->Self{
        Self{
            color,
            crowned:false
        }
    }

    pub fn crowned(p:GamePiece)->Self{
        Self{
            color:p.color,
            crowned:true
        }
    }
}

#[derive(Debug,Copy, Clone,PartialEq)]
pub struct Coordinate(pub usize,pub usize);

impl Coordinate {
    // self 作为一个参数，表示实例方法
    pub fn on_board(self)->bool{
        // 对象解构
        let Coordinate(x,y)=self;
        x<=7 && y<=7
    }

    pub fn jump_targets_from(&self)->impl Iterator<Item=Coordinate>{
        let mut  jumps=Vec::new();
        let Self(x,y)=*self;
        if y>=2{
            jumps.push(Self(x+2,y-2));
        }
        jumps.push(Self(x+2,y+2));
        if x>=2 && y>=2{
            jumps.push(Self(x-2,y-2));
        }
        if x>=2{
            jumps.push(Self(x-2,y+2));
        }
        jumps.into_iter()
    }
     pub fn move_targets_from(&self)->impl Iterator<Item=Coordinate>{
        
        let mut moves =Vec::new();
        let Self(x,y)=*self;
        if x>=1{
            moves.push(Self(x-1,y+1));
        }
        moves.push(Self(x+1,y+1));

        if y>=1{
            moves.push(Self(x+1,y-1));
        }
        if x>=1 && y>=1{
            moves.push(Self(x-1,y-1));
        }
        moves.into_iter()
     }
     
        
       
}