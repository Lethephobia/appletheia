-- Add down migration script here
DROP INDEX IF EXISTS transfers_status_idx;
DROP INDEX IF EXISTS transfers_to_account_idx;
DROP INDEX IF EXISTS transfers_from_account_idx;
DROP TABLE IF EXISTS transfers;
DROP INDEX IF EXISTS currency_issuances_status_idx;
DROP INDEX IF EXISTS currency_issuances_destination_account_idx;
DROP INDEX IF EXISTS currency_issuances_currency_idx;
DROP TABLE IF EXISTS currency_issuances;
DROP INDEX IF EXISTS currencies_status_idx;
DROP INDEX IF EXISTS currencies_owner_idx;
DROP INDEX IF EXISTS currencies_symbol_idx;
DROP TABLE IF EXISTS currencies;
DROP INDEX IF EXISTS accounts_status_idx;
DROP INDEX IF EXISTS accounts_currency_idx;
DROP INDEX IF EXISTS accounts_owner_idx;
DROP TABLE IF EXISTS accounts;
