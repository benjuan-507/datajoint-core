use std::convert::TryInto;

fn main() {
    let test = vec![4,5,6];
    let var = pack(test);
    println!("{:?}", var);

    unpack(var);
}

fn unpack <T: Pack + Copy> (mut blob: Vec<u8>) -> T{
    // Get Protocol
    let pos = get_zero_terminated_pos(&blob);
    blob.remove(pos);
    let mut protocol: Vec<u8> = blob;
    blob = protocol.split_off(pos);
    //println!("Protocol: {:?} Blob: {:?}", protocol, blob);

    return read_blob(blob);
}

fn read_blob <T: Pack + Copy> (mut blob: Vec<u8>) -> T{
    // Get Prefix
    let prefix = blob.get(0).cloned().unwrap();
    blob = blob.split_off(1);
    //println!("Prefix: {} Blob: {:?}", prefix, blob);

    match prefix{
        //b'\x0a'=>println!("{}", unpack_int(blob)),
        b'\x0a'=>return unpack_int(blob),
        //b'\x02'=>println!("{}", unpack_list(list)),
        _=>println!("Not Implemented")
    }
}

// def read_list(self):
// return list(self.read_blob(self.read_value()) for _ in range(self.read_value()))

// def read_value(self, dtype=None, count=1):
// if dtype is None:
//     dtype = 'uint32' if use_32bit_dims else 'uint64'
// data = np.frombuffer(self._blob, dtype=dtype, count=count, offset=self._pos)
// self._pos += data.dtype.itemsize * data.size
// return data[0] if count == 1 else data



fn pack<T: Pack>(obj: T) -> Vec<u8> {
    let mut blob: Vec<u8> = b"dj0\0".to_vec();
    blob.append(&mut pack_blob(obj));
    return blob;
}

#[inline]
fn pack_blob<T: Pack>(obj: T) -> Vec<u8> {
    let type_var = check_type(&obj);

    let packed_data: Vec<u8> = {
        match type_var {
            "i64" => obj.as_int().pack(),
            _ => obj.pack(),
        }
    };

    return packed_data;
}

trait Pack {
    //REMEMBER TO ADD YOUR FUNCTION HERE WHEN YOU WORK ON IT AND TO INCLUDE ALL
    fn pack(&self) -> Vec<u8>;

    fn as_string(self) -> String;
    fn as_int(self) -> i64;
}

macro_rules! pack_list {
    ($ty:ty) => {
        impl Pack for Vec<$ty> {
            fn pack(&self) -> Vec<u8> {
                let mut packed: Vec<u8> = b"\x02".to_vec();
                packed.push(self.len() as u8);
                packed.push(b'\0');
                
                for n in 0..self.len() {
                    packed.append( &mut pack_blob(*self.get(n).unwrap()));
                }

                return packed;
            }

            fn as_string(self) -> String {panic!()}
            fn as_int(self) -> i64 {panic!()}
        }
    }
}

//INCLUDE IMPLEMENTATION HERE IF PRIMITIVE TYPE
pack_list!(i64);
//pack_list!(f32);

impl Pack for i64 {
    #[inline]
    fn as_string(self) -> String {panic!()}
    fn as_int(self) -> i64 {self}

    fn pack(&self) -> Vec<u8> {
        let mut packed: Vec<u8> = b"\x0a".to_vec(); // Prefix
        let mut data: Vec<u8> = self.to_ne_bytes().to_vec(); // Data

        // Get size
        let mut n_bytes = self.to_ne_bytes().len(); // Size
        while n_bytes > 0 {
            if *data.get(n_bytes - 1).unwrap() == 0 {
                n_bytes = n_bytes - 1;
            }
            else {
                break;
            }
        }

        // Get rid of extra 0's
        data.truncate(n_bytes);

        packed.push(n_bytes as u8);
        packed.push(b'\0');

        packed.append(&mut data); // Data
        
        return packed;
    }
}

fn unpack_int(mut bytes:Vec<u8>) -> i64{
    // Get n_byte
    let pos = get_zero_terminated_pos(&bytes);
    bytes.remove(pos);
    let mut n_bytes: Vec<u8> = bytes;
    bytes = n_bytes.split_off(pos);

    let mut i = 8 - *n_bytes.get(0).unwrap();
    while i > 0 {
        bytes.push(b'\0');
        i = i - 1;
    }

    let byte_arr = bytes.try_into().unwrap();
    i64::from_ne_bytes(byte_arr)
}

fn check_type<T>(_obj: &T) -> &str {
    let type_var: &str = std::any::type_name::<T>();
    //println!("{}", std::any::type_name::<T>());

    return type_var;
}

fn get_zero_terminated_pos (blob: &Vec<u8>) -> usize{
    let mut iter = blob.iter();
    return iter.position(|x| x == &b'\0').unwrap();
}