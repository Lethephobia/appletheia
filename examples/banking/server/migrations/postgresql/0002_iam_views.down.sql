-- user_private_info read model
DROP INDEX IF EXISTS user_private_info_identities_email_idx;
DROP INDEX IF EXISTS user_private_info_identities_user_idx;
DROP TABLE IF EXISTS user_private_info_identities;
DROP INDEX IF EXISTS user_private_infos_status_idx;
DROP INDEX IF EXISTS user_private_infos_username_idx;
DROP TABLE IF EXISTS user_private_infos;
