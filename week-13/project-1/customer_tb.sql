--
-- PostgreSQL database dump
--

\restrict JOlzpBLB4Y2KkrsYMQzhiey1Y4Ha2q5okv44yZKwogezRLVdqmvh1622HIyqhwW

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name character varying(50) NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(50) NOT NULL,
    c_mobile character varying(15) NOT NULL,
    eid integer NOT NULL,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: customer customer_data_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_data_id_fkey FOREIGN KEY (data_id) REFERENCES public.dataplan(data_id);


--
-- PostgreSQL database dump complete
--

\unrestrict JOlzpBLB4Y2KkrsYMQzhiey1Y4Ha2q5okv44yZKwogezRLVdqmvh1622HIyqhwW

