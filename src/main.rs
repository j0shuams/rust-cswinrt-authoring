use bindings::authoring_demo::*;
use bindings::coords::*;
use bindings::*;
use futures::executor::block_on;

fn main() -> windows::Result<()> {
    let example = Example::new()?;
    example.set_sample_property(42)?;
    println!("{}", example.sample_property()?);
    let hello = Example::say_hello()?.to_string();
    println!("{}", hello);

    let folder_enumerator = FolderEnumeration::new()?;
    let future = folder_enumerator.get_files_and_folders_async()?;
    block_on(future)?;
    let files = folder_enumerator.all_files()?.to_string();
    println!("{}", files);

    println!("--------------------");
    let x : Coord = Coord::new()?;
    let y : Coord = Coord::create_coord(39.0, 80.0)?;
    println!("Distance between {} and {}", x.to_string()?.to_string(), y.to_string()?.to_string());
    println!("Expect : 89");
    println!("Actual : {}", x.distance(y)?);

    Ok(())
}
