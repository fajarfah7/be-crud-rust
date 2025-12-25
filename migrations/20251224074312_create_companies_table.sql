-- Add migration script here
CREATE TABLE public.companies (
    id uuid NOT NULL,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    code VARCHAR(10) NOT NULL,
    phone_number VARCHAR(20),
    address text,
    created_at timestamp with time zone NOT NULL
);
