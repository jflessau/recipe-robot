-- user 
define table user schemafull;
define field password_hash on table user type string assert string::len($value) >= 60;
define field created_at on table user type datetime default time::now() readonly;

-- invite
define table invite schemafull;
define field initial_charges on table invite type int default 1 assert $value >= 0 and $value <= 1000 readonly;
define field used_charges on table invite type int default 0 assert $value >= 0;

create invite:04905b58a3402deee88cb8a5cb2ee41f556afc9a603cfe7b515c7847ffdcd551338dc76cd662c6bc705066e1ae654abe5c48dcfb615a8201d40edd57 set
  initial_charges = 16,
  used_charges = 0;

-- emits
define table emits type relation from user to invite enforced schemafull;
define field created_at on table emits type datetime default time::now() readonly;
define index unique_emits on table emits columns in, out unique;

-- spawns
define table spawns type relation from invite to user enforced schemafull;
define field created_at on table spawns type datetime default time::now() readonly;
define index unique_spawned on table spawns columns in, out unique;

-- recipe 
define table recipe schemafull;
define field text on table recipe type string assert string::len($value) > 1 readonly;

-- ingredient
define table ingredient schemafull;
define field name on table ingredient type string assert string::len($value) > 0 readonly;
define field probably_at_home on table ingredient type bool default false readonly;

-- requires
define table requires type relation from recipe to ingredient enforced schemafull;
define field quantity on table requires type int default 1 assert $value > 0 readonly;
define field unit on table requires type string assert string::len($value) > 0 readonly;
define index unique_requires on table requires columns in, out unique;

-- submits
define table submits type relation from user to recipe enforced schemafull;
define field created_at on table submits type datetime default time::now() readonly;
define index unique_submits on table submits columns in, out unique;

-- seeks
define table seeks type relation from user to ingredient enforced schemafull;
define field vendor on table seeks type "rewe" | "edeka" | "lidl" | "aldi" | "netto" | "penny" | "real" | "kaufland" readonly;
define field created_at on table seeks type datetime default time::now() readonly;
define field unit on table seeks type string assert string::len($value) > 0 readonly;
define field quantity on table seeks type int assert $value > 0 readonly;
define index unique_seeks on table seeks columns in, out unique;

-- item
define table item schemafull;
define field name on table item type string assert string::len($value) > 0 readonly;
define field price_cent on table item type int assert $value >= 0 readonly;
define field quantity on table item type option<string> readonly;
define field url on table item type option<string> readonly;
define field image_url on table item type option<string> readonly;

-- matches
define table matches type relation from item to ingredient enforced schemafull;
define field created_at on table matches type datetime default time::now() readonly;
define field vendor on table matches type "rewe" | "edeka" | "lidl" | "aldi" | "netto" | "penny" | "real" | "kaufland" readonly;
define index unique_matches on table matches columns in, out unique;

-- cash_flow 
define table cash_flow schemafull;
define field created_at on table cash_flow type datetime default time::now() readonly;
define field amount on table cash_flow type int readonly;
define field origin on table cash_flow type "ai" | "private_assets" | "donation" readonly;

-- generates
define table generates type relation from user to cash_flow enforced schemafull;
define field created_at on table generates type datetime default time::now() readonly;
define index unique_generates on table generates columns in, out unique;





