-- user_public_profile read model
DROP INDEX IF EXISTS user_public_profiles_status_idx;
DROP INDEX IF EXISTS user_public_profiles_username_idx;
DROP TABLE IF EXISTS user_public_profiles;

-- user_organization_join_request_list read model
DROP INDEX IF EXISTS user_organization_join_request_list_org_idx;
DROP INDEX IF EXISTS user_org_join_request_list_scope_status_idx;
DROP INDEX IF EXISTS user_org_join_request_list_scope_created_at_idx;
DROP TABLE IF EXISTS user_organization_join_request_list_items;
DROP TABLE IF EXISTS user_organization_join_request_list_organizations;
DROP TABLE IF EXISTS user_organization_join_request_list_users;

-- organization_join_request_list read model
DROP INDEX IF EXISTS organization_join_request_list_requester_idx;
DROP INDEX IF EXISTS organization_join_request_list_scope_status_idx;
DROP INDEX IF EXISTS organization_join_request_list_scope_created_at_idx;
DROP TABLE IF EXISTS organization_join_request_list_items;
DROP TABLE IF EXISTS organization_join_request_list_users;
DROP TABLE IF EXISTS organization_join_request_list_organizations;

-- user_organization_invitation_list read model
DROP INDEX IF EXISTS user_organization_invitation_list_org_idx;
DROP INDEX IF EXISTS user_org_invitation_list_scope_status_idx;
DROP INDEX IF EXISTS user_org_invitation_list_scope_created_at_idx;
DROP TABLE IF EXISTS user_organization_invitation_list_items;
DROP TABLE IF EXISTS user_organization_invitation_list_organizations;
DROP TABLE IF EXISTS user_organization_invitation_list_users;

-- organization_invitation_list read model
DROP INDEX IF EXISTS organization_invitation_list_invitee_idx;
DROP INDEX IF EXISTS organization_invitation_list_scope_status_idx;
DROP INDEX IF EXISTS organization_invitation_list_scope_created_at_idx;
DROP TABLE IF EXISTS organization_invitation_list_items;
DROP TABLE IF EXISTS organization_invitation_list_users;
DROP TABLE IF EXISTS organization_invitation_list_organizations;

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
