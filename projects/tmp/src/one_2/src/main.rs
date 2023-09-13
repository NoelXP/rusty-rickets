

fn main() {
   let penguin_data = "\
   common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
   ";

   let records = penguin_data.lines();

   for (i, record) in records.enumerate() {
       if i == 0 || record.trim().len() == 0 {
           continue;
       }

       let fields = record.split(",");

       let field_data: Vec<&str> = fields.collect();

       let name = field_data[0];
       let maybe_length = field_data[1].parse::<f32>();

       let length = match maybe_length {
           Ok(l) => l,
           Err(_) => {
               println!("Problem parsing length for: {}", name);
               continue;
           }
       };

       println!("{}, {}cm", name, length);
   }




}