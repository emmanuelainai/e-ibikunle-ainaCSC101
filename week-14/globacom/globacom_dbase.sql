--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer,
    dno integer,
    dname character varying(42),
    dlocation character varying(60),
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer,
    pname character(8),
    pduration character(20),
    project_managerid integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name character(100) NOT NULL,
    dno integer NOT NULL,
    staff_sal character(200) NOT NULL,
    age integer,
    mobile character(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A       	9 MONTHS            	102
22	B       	14 MONTHS           	97
33	C       	16 MONTHS           	120
44	D       	25 MONTHS           	108
55	E       	9 MONTHS            	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	Mustapha Ali                                                                                        	3	175000                                                                                                                                                                                                  	32	08063285831    
107	Alokwe Martin                                                                                       	7	380000                                                                                                                                                                                                  	48	07090082812    
97	Dankade Aminat                                                                                      	5	550000                                                                                                                                                                                                  	40	09023688832    
108	Josiah Joshua                                                                                       	1	120000                                                                                                                                                                                                  	30	08053189131    
102	Makinde Mary                                                                                        	2	450000                                                                                                                                                                                                  	55	09023487830    
120	Adeleke Jane                                                                                        	4	200000                                                                                                                                                                                                  	38	07061045862    
122	Osahon Mark                                                                                         	6	320000                                                                                                                                                                                                  	44	08022289842    
104	Kuti Lawal                                                                                          	1	750000                                                                                                                                                                                                  	35	09145689842    
117	Suleman Ajayi                                                                                       	3	800000                                                                                                                                                                                                  	50	7030089981     
\.


--
-- PostgreSQL database dump complete
--

