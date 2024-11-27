-- user 
define table user schemafull;
define field id on table user type uuid;
define field created_at on table user type datetime default time::now();
define field password_hash on table user type string assert string::len($value) >= 60;
define field credit_balance on table user type int default 0 assert $value >= 0 and $value <= 1000000;
define field credits_spent on table user type int default 0 assert $value >= 0;

-- invite
define table invite schemafull;
define field id on table invite type uuid;
define field code on table invite type string assert string::len($value) == 64;
define field user_id on table invite type uuid;
define field initial_charges on table invite type int default 1 assert $value >= 0 and $value <= 1000;
define field used_charges on table invite type int default 0 assert $value >= 0 and $value;

-- earned
define table earned type relation from user to invite enforced schemafull;
define field created_at on table earned type datetime default time::now();
define index unique_earned on table earned columns in, out unique;

