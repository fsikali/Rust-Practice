/*
- Another advantage of using an enum rather than a struct:
- Each variant can have different types and amounts of associated data. 
- If we wanted to store V4 addresses as four u8 values but still express V6 addresses
  as one String value, we wouldn't be able to with a struct
- Enums handle this case with ease:
*/

enum IpAddr { 
    V4(u8, u8, u8, u8),
    V6(String),
} 

