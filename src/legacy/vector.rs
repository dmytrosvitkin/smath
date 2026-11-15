use core::ops::{Add,AddAssign,Div,DivAssign,Mul,MulAssign,Neg,Sub,SubAssign};
use super::{sqrt,isqrt};
use super::{sin,cos,sinr,cosr,atan,atanr};

/// A 2D vector representing spatial coordinates (x, y).
#[derive(Debug,Copy,Clone,Default)]
pub struct Vec2{pub x:f32,pub y:f32}

impl Vec2{
        /// Create a new Vec2 (2D vector).
        #[inline]
        pub const fn new(x:f32,y:f32)->Self{
            Self{x,y}
        }

        /// Returns a vector with all components set to 0.0.
        #[inline]
        pub const fn zero()->Self{
            Self{x:0.0,y:0.0}
        }

        /// Returns a vector with all components set to 1.0.
        #[inline]
        pub const fn one()->Self{
            Self{x:1.0,y:1.0}
        }

        /// Sets the vector's components to the given values.
        #[inline]
        pub const fn set(&mut self,x:f32,y:f32){
            self.x=x;
            self.y=y
        }

        /// Shifts the vector's components by the given values.
        #[inline]
        pub const fn shift(&mut self,x:f32,y:f32){
            self.x+=x;
            self.y+=y
        }

        /// Calculates the length (magnitude) of the vector.
        #[inline]
        pub const fn length(&self)->f32{
            sqrt(self.sq_length())
        }

        /// Calculates the squared length of the vector.
        #[inline(always)]
        pub const fn sq_length(&self)->f32{
            self.x*self.x+self.y*self.y
        }

        /// Normalizes the vector in place.
        #[inline]
        pub const fn normalize(&mut self){
            let rcl:f32=self.sq_length();
            if rcl!=0.0{
                let rcl:f32=isqrt(rcl);
                self.x*=rcl;
                self.y*=rcl;
            }
        }

        /// Returns a new normalized vector.
        #[inline]
        pub const fn normalized(&self)->Self{
            let mut rcl:Vec2=*self;rcl.normalize();rcl
        }

        /// Calculates the dot product of two vectors.
        #[inline(always)]
        pub const fn dot(&self,other:Self)->f32{
            self.x*other.x+self.y*other.y
        }

        /// Calculates the distance between two vectors.
        #[inline]
        pub const fn distance(&self,other:Self)->f32{
            sqrt(self.sq_distance(other))
        }

        /// Calculates the squared distance between two vectors.
        #[inline(always)]
        pub const fn sq_distance(&self,other:Self)->f32{
            let dx:f32=self.x-other.x;
            let dy:f32=self.y-other.y;
            dx*dx+dy*dy
        }

        /// Linearly interpolates between this vector and another by a factor of t.
        #[inline]
        pub const fn lerp(&self,other:Self,t:f32)->Self{
            Self::new(
                self.x+(other.x-self.x)*t,
                self.y+(other.y-self.y)*t
            )
        }

        /// Clamps the vector components between a minimum and maximum bounding vector.
        #[inline]
        pub const fn clamp(&self,min:Self,max:Self)->Self{
            Self::new(
                if self.x<min.x{min.x}else if self.x>max.x{max.x}else{self.x},
                if self.y<min.y{min.y}else if self.y>max.y{max.y}else{self.y}
            )
        }

        /// Returns a vector containing the absolute value of each component.
        #[inline]
        pub const fn abs(&self)->Self{
            Self::new(
                if self.x<0.0{-self.x}else{self.x},
                if self.y<0.0{-self.y}else{self.y}
            )
        }

        /// Finds the midpoint between this vector and another.
        #[inline]
        pub const fn midpoint(&self,other:Self)->Self{
            Self::new(
                (self.x+other.x)*0.5,
                (self.y+other.y)*0.5
            )
        }

        /// Projects this vector onto another vector.
        #[inline]
        pub fn project(&self,other:Self)->Self{
            let d:f32=other.sq_length();
            if d==0.0{Self::new(0.0,0.0)}
            else{other*(self.dot(other)/d)}
        }

        /// Rejects this vector from another vector (perpendicular component).
        #[inline]
        pub fn reject(&self,other:Self)->Self{
            *self-self.project(other)
        }

        /// Reflects this vector off a surface defined by its normal vector.
        #[inline]
        pub fn reflect(&self,normal:Self)->Self{
            *self-normal*(2.0*self.dot(normal))
        }

        /// Returns a vector perpendicular to this one, rotated 90 degrees counter-clockwise.
        #[inline(always)]
        pub const fn perp(&self)->Self{
            Self::new(-self.y,self.x)
        }

        /// Calculates the angle of the vector in degrees relative to the positive X-axis.
        #[inline]
        pub const fn angle(&self)->f32{
            if self.x==0.0{
                if self.y>0.0{90.0}
                else if self.y<0.0{-90.0}
                else{0.0}
            }
            else{
                let rcl:f32=atan(self.y/self.x);
                if self.x<0.0{if self.y>=0.0{rcl+180.0}else{rcl-180.0}}
                else{rcl}
            }
        }

        /// Calculates the angle of the vector in degrees relative to the positive X-axis.
        #[inline(always)]
        pub const fn angle_deg(&self)->f32{
            self.angle()
        }

        /// Calculates the angle of the vector in pi-radians relative to the positive X-axis.
        #[inline]
        pub const fn angler(&self)->f32{
            if self.x==0.0{
                if self.y>0.0{0.5}
                else if self.y<0.0{-0.5}
                else{0.0}
            }
            else{
                let rcl:f32=atanr(self.y/self.x);
                if self.x<0.0{if self.y>=0.0{rcl+1.0}else{rcl-1.0}}
                else{rcl}
            }
        }

        /// Calculates the angle of the vector in pi-radians relative to the positive X-axis.
        #[inline]
        pub const fn angle_rad(&self)->f32{
            self.angler()
        }

        /// Rotates the 2D vector by a given angle (degrees, 45.0 = 45°).
        #[inline]
        pub const fn rotate(&self,angle:f32)->Self{
            let s:f32=sin(angle);
            let c:f32=cos(angle);
            Self::new(self.x*c-self.y*s,self.x*s+self.y*c)
        }

        /// Rotates the 2D vector by a given angle (radians, 1.0 = π).
        #[inline]
        pub const fn rotater(&self,angle:f32)->Self{
            let s:f32=sinr(angle);
            let c:f32=cosr(angle);
            Self::new(self.x*c-self.y*s,self.x*s+self.y*c)
        }

        /// Rotates the 2D vector by a given angle (degrees, 45.0 = 45°).
        #[inline]
        pub const fn rotate_deg(&self,angle:f32)->Self{
            self.rotate(angle)
        }

        /// Rotates the 2D vector by a given angle (radians, 1.0 = π).
        #[inline]
        pub const fn rotate_rad(&self,angle:f32)->Self{
            self.rotater(angle)
        }
    }

    impl Add for Vec2{
        type Output=Self;
        #[inline]
        fn add(self,other:Self)->Self{
            Self::new(self.x+other.x,self.y+other.y)
        }
    }

    impl Sub for Vec2{
        type Output=Self;
        #[inline]
        fn sub(self,other:Self)->Self{
            Self::new(self.x-other.x,self.y-other.y)
        }
    }

    impl Mul<f32>for Vec2{
        type Output=Self;
        #[inline]
        fn mul(self,scalar:f32)->Self{
            Self::new(self.x*scalar,self.y*scalar)
        }
    }

    impl Mul<Vec2>for Vec2{
        type Output=Self;
        #[inline]
        fn mul(self,other:Self)->Self{
            Self::new(self.x*other.x,self.y*other.y)
        }
    }

    impl Div<f32>for Vec2{
        type Output=Self;
        #[inline]
        fn div(self,scalar:f32)->Self{
            Self::new(self.x/scalar,self.y/scalar)
        }
    }

    impl Div<Vec2>for Vec2{
        type Output=Self;
        #[inline]
        fn div(self,other:Self)->Self{
            Self::new(self.x/other.x,self.y/other.y)
        }
    }

    impl Neg for Vec2{
        type Output=Self;
        #[inline]
        fn neg(self)->Self{
            Self::new(-self.x,-self.y)
        }
    }

    impl AddAssign for Vec2{
        #[inline]
        fn add_assign(&mut self,other:Self){
            self.x+=other.x;
            self.y+=other.y;
        }
    }

    impl SubAssign for Vec2{
        #[inline]
        fn sub_assign(&mut self,other:Self){
            self.x-=other.x;
            self.y-=other.y;
        }
    }

    impl MulAssign<f32>for Vec2{
        #[inline]
        fn mul_assign(&mut self,scalar:f32){
            self.x*=scalar;
            self.y*=scalar;
        }
    }

    impl MulAssign<Vec2>for Vec2{
        #[inline]
        fn mul_assign(&mut self,other:Self){
            self.x*=other.x;
            self.y*=other.y;
        }
    }

    impl DivAssign<f32>for Vec2{
        #[inline]
        fn div_assign(&mut self,scalar:f32){
            self.x/=scalar;
            self.y/=scalar;
        }
    }

    impl DivAssign<Vec2>for Vec2{
        #[inline]
        fn div_assign(&mut self,other:Self){
            self.x/=other.x;
            self.y/=other.y;
        }
    }
/// A 3D vector representing spatial coordinates (x, y, z).
#[derive(Debug,Copy,Clone,Default)]
pub struct Vec3{pub x:f32,pub y:f32,pub z:f32}

impl Vec3{
        /// Create a new Vec3 (3D vector).
        #[inline]
        pub const fn new(x:f32,y:f32,z:f32)->Self{
            Self{x,y,z}
        }

        /// Returns a vector with all components set to 0.0.
        #[inline]
        pub const fn zero()->Self{
            Self{x:0.0,y:0.0,z:0.0}
        }

        /// Returns a vector with all components set to 1.0.
        #[inline]
        pub const fn one()->Self{
            Self{x:1.0,y:1.0,z:1.0}
        }

        /// Sets the vector's components to the given values.
        #[inline]
        pub const fn set(&mut self,x:f32,y:f32,z:f32){
            self.x=x;
            self.y=y;
            self.z=z
        }

        /// Shifts the vector's components by the given values.
        #[inline]
        pub const fn shift(&mut self,x:f32,y:f32,z:f32){
            self.x+=x;
            self.y+=y;
            self.z+=z
        }

        /// Calculates the length (magnitude) of the vector.
        #[inline]
        pub const fn length(&self)->f32{
            sqrt(self.sq_length())
        }

        /// Calculates the squared length of the vector.
        #[inline(always)]
        pub const fn sq_length(&self)->f32{
            self.x*self.x+self.y*self.y+self.z*self.z
        }

        /// Normalizes the vector in place.
        #[inline]
        pub const fn normalize(&mut self){
            let rcl:f32=self.sq_length();
            if rcl!=0.0{
                let rcl:f32=isqrt(rcl);
                self.x*=rcl;
                self.y*=rcl;
                self.z*=rcl;
            }
        }

        /// Returns a new normalized vector.
        #[inline]
        pub const fn normalized(&self)->Self{
            let mut rcl:Vec3=*self;rcl.normalize();rcl
        }

        /// Calculates the dot product of two vectors.
        #[inline(always)]
        pub const fn dot(&self,other:Self)->f32{
            self.x*other.x+self.y*other.y+self.z*other.z
        }

        /// Calculates the distance between two vectors.
        #[inline]
        pub const fn distance(&self,other:Self)->f32{
            sqrt(self.sq_distance(other))
        }

        /// Calculates the squared distance between two vectors.
        #[inline(always)]
        pub const fn sq_distance(&self,other:Self)->f32{
            let dx:f32=self.x-other.x;
            let dy:f32=self.y-other.y;
            let dz:f32=self.z-other.z;
            dx*dx+dy*dy+dz*dz
        }

        /// Linearly interpolates between this vector and another by a factor of t.
        #[inline]
        pub const fn lerp(&self,other:Self,t:f32)->Self{
            Self::new(
                self.x+(other.x-self.x)*t,
                self.y+(other.y-self.y)*t,
                self.z+(other.z-self.z)*t
            )
        }

        /// Clamps the vector components between a minimum and maximum bounding vector.
        #[inline]
        pub const fn clamp(&self,min:Self,max:Self)->Self{
            Self::new(
                if self.x<min.x{min.x}else if self.x>max.x{max.x}else{self.x},
                if self.y<min.y{min.y}else if self.y>max.y{max.y}else{self.y},
                if self.z<min.z{min.z}else if self.z>max.z{max.z}else{self.z}
            )
        }

        /// Returns a vector containing the absolute value of each component.
        #[inline]
        pub const fn abs(&self)->Self{
            Self::new(
                if self.x<0.0{-self.x}else{self.x},
                if self.y<0.0{-self.y}else{self.y},
                if self.z<0.0{-self.z}else{self.z}
            )
        }

        /// Finds the midpoint between this vector and another.
        #[inline]
        pub const fn midpoint(&self,other:Self)->Self{
            Self::new(
                (self.x+other.x)*0.5,
                (self.y+other.y)*0.5,
                (self.z+other.z)*0.5
            )
        }

        /// Projects this vector onto another vector.
        #[inline]
        pub fn project(&self,other:Self)->Self{
            let d:f32=other.sq_length();
            if d==0.0{Self::new(0.0,0.0,0.0)}
            else{other*(self.dot(other)/d)}
        }

        /// Rejects this vector from another vector (perpendicular component).
        #[inline]
        pub fn reject(&self,other:Self)->Self{
            *self-self.project(other)
        }

        /// Reflects this vector off a surface defined by its normal vector.
        #[inline]
        pub fn reflect(&self,normal:Self)->Self{
            *self-normal*(2.0*self.dot(normal))
        }

        /// Calculates the cross product of two vectors.
        #[inline]
        pub const fn cross(&self,other:Self)->Self{
            Self::new(
                self.y*other.z-self.z*other.y,
                self.z*other.x-self.x*other.z,
                self.x*other.y-self.y*other.x
            )
        }

        /// Flattens the 3D vector by discarding the Z-axis (orthographic projection).
        #[inline]
        pub const fn flat(&self)->Vec2{
            Vec2::new(self.x,self.y)
        }

        /// Flattens the 3D vector onto the 2D XY plane, scaling the horizontal components based on the vertical Z tilt.
        #[inline]
        pub const fn sp_flat(&self)->Vec2{
            let rcl:f32=self.sq_length();
            let z_sq:f32=self.z*self.z;
            if z_sq>=rcl{Vec2::zero()}
            else{let rcl:f32=sqrt(1.0-(z_sq/rcl));Vec2::new(self.x*rcl,self.y*rcl)}
        }

        /// Flattens the vector by dividing X and Y by Z (perspective projection).
        #[inline]
        pub const fn persp(&self)->Vec2{
            if self.z==0.0{Vec2::zero()}
            else{Vec2::new(self.x/self.z,self.y/self.z)}
        }
    }

    impl Add for Vec3{
        type Output=Self;
        #[inline]
        fn add(self,other:Self)->Self{
            Self::new(self.x+other.x,self.y+other.y,self.z+other.z)
        }
    }

    impl Sub for Vec3{
        type Output=Self;
        #[inline]
        fn sub(self,other:Self)->Self{
            Self::new(self.x-other.x,self.y-other.y,self.z-other.z)
        }
    }

    impl Mul<f32>for Vec3{
        type Output=Self;
        #[inline]
        fn mul(self,scalar:f32)->Self{
            Self::new(self.x*scalar,self.y*scalar,self.z*scalar)
        }
    }

    impl Mul<Vec3>for Vec3{
        type Output=Self;
        #[inline]
        fn mul(self,other:Self)->Self{
            Self::new(self.x*other.x,self.y*other.y,self.z*other.z)
        }
    }

    impl Div<f32>for Vec3{
        type Output=Self;
        #[inline]
        fn div(self,scalar:f32)->Self{
            Self::new(self.x/scalar,self.y/scalar,self.z/scalar)
        }
    }

    impl Div<Vec3>for Vec3{
        type Output=Self;
        #[inline]
        fn div(self,other:Self)->Self{
            Self::new(self.x/other.x,self.y/other.y,self.z/other.z)
        }
    }

    impl Neg for Vec3{
        type Output=Self;
        #[inline]
        fn neg(self)->Self{
            Self::new(-self.x,-self.y,-self.z)
        }
    }

    impl AddAssign for Vec3{
        #[inline]
        fn add_assign(&mut self,other:Self){
            self.x+=other.x;
            self.y+=other.y;
            self.z+=other.z;
        }
    }

    impl SubAssign for Vec3{
        #[inline]
        fn sub_assign(&mut self,other:Self){
            self.x-=other.x;
            self.y-=other.y;
            self.z-=other.z;
        }
    }

    impl MulAssign<f32>for Vec3{
        #[inline]
        fn mul_assign(&mut self,scalar:f32){
            self.x*=scalar;
            self.y*=scalar;
            self.z*=scalar;
        }
    }

    impl MulAssign<Vec3>for Vec3{
        #[inline]
        fn mul_assign(&mut self,other:Self){
            self.x*=other.x;
            self.y*=other.y;
            self.z*=other.z;
        }
    }

    impl DivAssign<f32>for Vec3{
        #[inline]
        fn div_assign(&mut self,scalar:f32){
            self.x/=scalar;
            self.y/=scalar;
            self.z/=scalar;
        }
    }

    impl DivAssign<Vec3>for Vec3{
        #[inline]
        fn div_assign(&mut self,other:Self){
            self.x/=other.x;
            self.y/=other.y;
            self.z/=other.z;
        }
    }

/// A 4D vector representing spatial coordinates (x, y, z, w).
#[derive(Debug,Copy,Clone,Default)]
pub struct Vec4{pub x:f32,pub y:f32,pub z:f32,pub w:f32}

impl Vec4{
        /// Create a new Vec4 (4D vector).
        #[inline]
        pub const fn new(x:f32,y:f32,z:f32,w:f32)->Self{
            Self{x,y,z,w}
        }

        /// Returns a vector with all components set to 0.0.
        #[inline]
        pub const fn zero()->Self{
            Self{x:0.0,y:0.0,z:0.0,w:0.0}
        }

        /// Returns a vector with all components set to 1.0.
        #[inline]
        pub const fn one()->Self{
            Self{x:1.0,y:1.0,z:1.0,w:1.0}
        }

        /// Sets the vector's components to the given values.
        #[inline]
        pub const fn set(&mut self,x:f32,y:f32,z:f32,w:f32){
            self.x=x;
            self.y=y;
            self.z=z;
            self.w=w
        }

        /// Shifts the vector's components by the given values.
        #[inline]
        pub const fn shift(&mut self,x:f32,y:f32,z:f32,w:f32){
            self.x+=x;
            self.y+=y;
            self.z+=z;
            self.w+=w
        }

        /// Calculates the length (magnitude) of the vector.
        #[inline]
        pub const fn length(&self)->f32{
            sqrt(self.sq_length())
        }

        /// Calculates the squared length of the vector.
        #[inline(always)]
        pub const fn sq_length(&self)->f32{
            self.x*self.x+self.y*self.y+self.z*self.z+self.w*self.w
        }

        /// Normalizes the vector in place.
        #[inline]
        pub const fn normalize(&mut self){
            let rcl:f32=self.sq_length();
            if rcl!=0.0{
                let rcl:f32=isqrt(rcl);
                self.x*=rcl;
                self.y*=rcl;
                self.z*=rcl;
                self.w*=rcl;
            }
        }

        /// Returns a new normalized vector.
        #[inline]
        pub const fn normalized(&self)->Self{
            let mut rcl:Vec4=*self;rcl.normalize();rcl
        }

        /// Calculates the dot product of two vectors.
        #[inline(always)]
        pub const fn dot(&self,other:Self)->f32{
            self.x*other.x+self.y*other.y+self.z*other.z+self.w*other.w
        }

        /// Calculates the distance between two vectors.
        #[inline]
        pub const fn distance(&self,other:Self)->f32{
            sqrt(self.sq_distance(other))
        }

        /// Calculates the squared distance between two vectors.
        #[inline(always)]
        pub const fn sq_distance(&self,other:Self)->f32{
            let dx:f32=self.x-other.x;
            let dy:f32=self.y-other.y;
            let dz:f32=self.z-other.z;
            let dw:f32=self.w-other.w;
            dx*dx+dy*dy+dz*dz+dw*dw
        }

        /// Linearly interpolates between this vector and another by a factor of t.
        #[inline]
        pub const fn lerp(&self,other:Self,t:f32)->Self{
            Self::new(
                self.x+(other.x-self.x)*t,
                self.y+(other.y-self.y)*t,
                self.z+(other.z-self.z)*t,
                self.w+(other.w-self.w)*t
            )
        }

        /// Clamps the vector components between a minimum and maximum bounding vector.
        #[inline]
        pub const fn clamp(&self,min:Self,max:Self)->Self{
            Self::new(
                if self.x<min.x{min.x}else if self.x>max.x{max.x}else{self.x},
                if self.y<min.y{min.y}else if self.y>max.y{max.y}else{self.y},
                if self.z<min.z{min.z}else if self.z>max.z{max.z}else{self.z},
                if self.w<min.w{min.w}else if self.w>max.w{max.w}else{self.w}
            )
        }

        /// Returns a vector containing the absolute value of each component.
        #[inline]
        pub const fn abs(&self)->Self{
            Self::new(
                if self.x<0.0{-self.x}else{self.x},
                if self.y<0.0{-self.y}else{self.y},
                if self.z<0.0{-self.z}else{self.z},
                if self.w<0.0{-self.w}else{self.w}
            )
        }

        /// Finds the midpoint between this vector and another.
        #[inline]
        pub const fn midpoint(&self,other:Self)->Self{
            Self::new(
                (self.x+other.x)*0.5,
                (self.y+other.y)*0.5,
                (self.z+other.z)*0.5,
                (self.w+other.w)*0.5
            )
        }

        /// Projects this vector onto another vector.
        #[inline]
        pub fn project(&self,other:Self)->Self{
            let d:f32=other.sq_length();
            if d==0.0{Self::new(0.0,0.0,0.0,0.0)}
            else{other*(self.dot(other)/d)}
        }

        /// Rejects this vector from another vector (perpendicular component).
        #[inline]
        pub fn reject(&self,other:Self)->Self{
            *self-self.project(other)
        }

        /// Reflects this vector off a surface defined by its normal vector.
        #[inline]
        pub fn reflect(&self,normal:Self)->Self{
            *self-normal*(2.0*self.dot(normal))
        }
    }

    impl Add for Vec4{
        type Output=Self;
        #[inline]
        fn add(self,other:Self)->Self{
            Self::new(self.x+other.x,self.y+other.y,self.z+other.z,self.w+other.w)
        }
    }

    impl Sub for Vec4{
        type Output=Self;
        #[inline]
        fn sub(self,other:Self)->Self{
            Self::new(self.x-other.x,self.y-other.y,self.z-other.z,self.w-other.w)
        }
    }

    impl Mul<f32>for Vec4{
        type Output=Self;
        #[inline]
        fn mul(self,scalar:f32)->Self{
            Self::new(self.x*scalar,self.y*scalar,self.z*scalar,self.w*scalar)
        }
    }

    impl Mul<Vec4>for Vec4{
        type Output=Self;
        #[inline]
        fn mul(self,other:Self)->Self{
            Self::new(self.x*other.x,self.y*other.y,self.z*other.z,self.w*other.w)
        }
    }

    impl Div<f32>for Vec4{
        type Output=Self;
        #[inline]
        fn div(self,scalar:f32)->Self{
            Self::new(self.x/scalar,self.y/scalar,self.z/scalar,self.w/scalar)
        }
    }

    impl Div<Vec4>for Vec4{
        type Output=Self;
        #[inline]
        fn div(self,other:Self)->Self{
            Self::new(self.x/other.x,self.y/other.y,self.z/other.z,self.w/other.w)
        }
    }

    impl Neg for Vec4{
        type Output=Self;
        #[inline]
        fn neg(self)->Self{
            Self::new(-self.x,-self.y,-self.z,-self.w)
        }
    }

    impl AddAssign for Vec4{
        #[inline]
        fn add_assign(&mut self,other:Self){
            self.x+=other.x;
            self.y+=other.y;
            self.z+=other.z;
            self.w+=other.w;
        }
    }

    impl SubAssign for Vec4{
        #[inline]
        fn sub_assign(&mut self,other:Self){
            self.x-=other.x;
            self.y-=other.y;
            self.z-=other.z;
            self.w-=other.w;
        }
    }

    impl MulAssign<f32>for Vec4{
        #[inline]
        fn mul_assign(&mut self,scalar:f32){
            self.x*=scalar;
            self.y*=scalar;
            self.z*=scalar;
            self.w*=scalar;
        }
    }

    impl MulAssign<Vec4>for Vec4{
        #[inline]
        fn mul_assign(&mut self,other:Self){
            self.x*=other.x;
            self.y*=other.y;
            self.z*=other.z;
            self.w*=other.w;
        }
    }

    impl DivAssign<f32>for Vec4{
        #[inline]
        fn div_assign(&mut self,scalar:f32){
            self.x/=scalar;
            self.y/=scalar;
            self.z/=scalar;
            self.w/=scalar;
        }
    }

    impl DivAssign<Vec4>for Vec4{
        #[inline]
        fn div_assign(&mut self,other:Self){
            self.x/=other.x;
            self.y/=other.y;
            self.z/=other.z;
            self.w/=other.w;
        }
    }
