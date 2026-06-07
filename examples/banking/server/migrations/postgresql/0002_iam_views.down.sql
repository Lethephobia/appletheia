-- user_public_profile read model
DROP INDEX IF EXISTS user_public_profiles_status_idx;
DROP INDEX IF EXISTS user_public_profiles_username_idx;
DROP TABLE IF EXISTS user_public_profiles;

-- user_private_info read model
DROP INDEX IF EXISTS user_private_info_organization_memberships_organization_idx;
DROP INDEX IF EXISTS user_private_info_organization_memberships_user_idx;
DROP TABLE IF EXISTS user_private_info_organization_memberships;
DROP INDEX IF EXISTS user_private_info_organizations_handle_idx;
DROP TABLE IF EXISTS user_private_info_organizations;
DROP INDEX IF EXISTS user_private_info_identities_email_idx;
DROP INDEX IF EXISTS user_private_info_identities_user_idx;
DROP TABLE IF EXISTS user_private_info_identities;
DROP INDEX IF EXISTS user_private_infos_status_idx;
DROP INDEX IF EXISTS user_private_infos_username_idx;
DROP TABLE IF EXISTS user_private_infos;
