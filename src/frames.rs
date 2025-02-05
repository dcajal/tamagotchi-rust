pub const FRAMES_IDLE: [&str; 2] = [
    r#"
      /\___/\
     ( o   o )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    "#,
    r#"
      /\___/\
     ( ^   ^ )
     (  =^=  )
    (( \___/ ))
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    "#,
];

pub const FRAMES_DEAD: [&str; 1] = [r#"
      /\___/\
     ( X   X )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    "#];

pub const FRAMES_EAT: [&str; 2] = [
    r#"
      /\___/\
     ( o   o )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    *
    "#,
    r#"
      /\___/\
     ( ^   ^ )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
      *
    "#,
];

pub const FRAMES_PLAY: [&str; 2] = [
    r#"
      /\___/\
     ( o   o )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
    _oo_   _oo_
    "#,
    r#"
      /\___/\
     ( o   o )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
   __||__ __||__
     oo     oo
    "#,
];

pub const FRAMES_SLEEP: [&str; 2] = [
    r#"
        z
      /\___/\
     ( -   - )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    "#,
    r#"
        zzZ
      /\___/\
     ( -   - )
     (  =^=  )
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    "#,
];

pub const FRAMES_BATH: [&str; 2] = [
    r#"
      /\___/\
     ( o   o )
     (  =^=  )    ~
    /( \___/ )\  ~
   /  )     (  \~
  (__/       \__)
     ||     ||
     oo     oo
    "#,
    r#"
      /\___/\    ~
     ( ^   ^ ) ~
     (  =^=  )~
    /( \___/ )\
   /  )     (  \
  (__/       \__)
     ||     ||
     oo     oo
    "#,
];
