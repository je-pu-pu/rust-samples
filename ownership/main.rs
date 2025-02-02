fn x( value: i32 )
{
    println!( "x() value: {}", value );
}

fn y( value: String )
{
    println!( "y() value: {}", value );
}

fn z( value: &String )
{
    println!( "z() value: {}", value );

    // mut がついていないので変更はできない
    // value = String::from( "string was changed." )
}

fn z2( value: &mut String )
{
    println!( "z2() value: {}", value );

    // mut がついているので変更はできる
    value.clear();
    value.push_str( "string was changed." )
}

fn main()
{

    let a = 123; // i32 は Copy トレイトを実装しているため、コピー可能

    x( a ); //  x の所有権は関数にムーブされるが、

    // 元の変数も使える
    println!( "a: {}", a );


    // 文字列の場合は、
    let s = String::from( "this is String" );

    y( s ); // s の所有権は関数にムーブされる

    // 元の変数は使おうとするとコンパイル時エラー
    // println!( "s: {}", s );


    let s2 = String::from( "this is String 2" );

    // 参照 ( 借用 ) で渡せば、
    z( & s2 );

    // 元の変数も使える
    println!( "s2: {}", s2 );


    // さらに、 mut をつければ、関数内で変更できる
    let mut s3 = String::from( "this is String 3" );

    z2( &mut s3 );

    println!( "s3: {}", s3 );
}