z_owned_string_t s, s1;
z_string_copy_from_str(&s, "Hello, world!");
// notice that the prototype of z_string_clone is
// void z_string_clone(z_owned_string_t* dst, const z_loaned_string_t* src);
// I.e. the only way to pass the source string is by loaning it
z_string_clone(&s1, z_loan(s));
//...
z_drop(z_move(s));
z_drop(z_move(s1));
