-- user_public_profile read model
DROP INDEX IF EXISTS user_public_profiles_status_idx;
DROP INDEX IF EXISTS user_public_profiles_username_idx;
DROP TABLE IF EXISTS user_public_profiles;

-- organization_management_info read model
DROP INDEX IF EXISTS organization_management_infos_owner_user_idx;
DROP INDEX IF EXISTS organization_management_infos_handle_idx;
DROP TABLE IF EXISTS organization_management_infos;
DROP INDEX IF EXISTS organization_management_info_owner_users_username_idx;
DROP TABLE IF EXISTS organization_management_info_owner_users;

-- organization_internal_info read model
DROP INDEX IF EXISTS organization_internal_infos_handle_idx;
DROP TABLE IF EXISTS organization_internal_infos;

-- public_organization_list read model
DROP INDEX IF EXISTS public_organization_list_items_created_at_idx;
DROP INDEX IF EXISTS public_organization_list_items_handle_contains_idx;
DROP INDEX IF EXISTS public_organization_list_items_handle_idx;
DROP TABLE IF EXISTS public_organization_list_items;

-- public_user_list read model
DROP INDEX IF EXISTS public_user_list_items_status_created_at_idx;
DROP INDEX IF EXISTS public_user_list_items_username_contains_idx;
DROP INDEX IF EXISTS public_user_list_items_username_idx;
DROP TABLE IF EXISTS public_user_list_items;

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
