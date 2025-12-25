-- Add migration script here
CREATE TABLE public.users (
    id uuid NOT NULL,
    name VARCHAR(100) NOT NULL,
    username VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20),
    encrypted_password VARCHAR(255),
    created_at timestamp with time zone NOT NULL
);