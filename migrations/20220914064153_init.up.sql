CREATE SCHEMA "dashboard";

CREATE TYPE "dashboard"."role" AS ENUM (
  'admin',
  'cashier'
);

CREATE TABLE "dashboard"."cinema" (
  "id" uuid PRIMARY KEY,
  "addres" varchar NOT NULL,
  "coordinates" point NOT NULL,
  "admin_id" uuid
);

CREATE TABLE "dashboard"."photo" (
  "id" uuid PRIMARY KEY,
  "cinema_id" uuid,
  "link" varchar
);

CREATE TABLE "dashboard"."hall" (
  "id" uuid PRIMARY KEY,
  "cinema_id" uuid
);

CREATE TABLE "dashboard"."seat" (
  "id" uuid PRIMARY KEY,
  "hall_id" uuid
);

CREATE TABLE "dashboard"."user" (
  "id" uuid PRIMARY KEY,
  "login" varchar,
  "role" dashboard.role
);

CREATE TABLE "dashboard"."password" (
  "id" uuid PRIMARY KEY,
  "user_id" uuid,
  "password" varchar
);

CREATE TABLE "movie" (
  "id" uuid PRIMARY KEY
);

CREATE TABLE "session" (
  "id" uuid PRIMARY KEY,
  "movie_id" uuid,
  "hall_id" uuid,
  "time" timestamp,
  "diration" int
);

CREATE TABLE "order" (
  "id" uuid PRIMARY KEY,
  "session_id" uuid,
  "price" int
);

COMMENT ON COLUMN "dashboard"."photo"."link" IS 'Link in the S3 or other similar storedge';

ALTER TABLE "dashboard"."cinema" ADD CONSTRAINT dashboard_cinema_admin_id FOREIGN KEY ("admin_id") REFERENCES "dashboard"."user" ("id");

ALTER TABLE "dashboard"."photo" ADD CONSTRAINT dashboard_photo_cinema_id FOREIGN KEY ("cinema_id") REFERENCES "dashboard"."cinema" ("id");

ALTER TABLE "dashboard"."hall" ADD CONSTRAINT dashboard_hall_cinema_id FOREIGN KEY ("cinema_id") REFERENCES "dashboard"."cinema" ("id");

ALTER TABLE "dashboard"."seat" ADD CONSTRAINT dashboard_seat_hall_id FOREIGN KEY ("hall_id") REFERENCES "dashboard"."hall" ("id");

ALTER TABLE "dashboard"."password" ADD CONSTRAINT dashboard_password_user_id FOREIGN KEY ("user_id") REFERENCES "dashboard"."user" ("id");

ALTER TABLE "session" ADD CONSTRAINT session_movie_id FOREIGN KEY ("movie_id") REFERENCES "movie" ("id");

ALTER TABLE "session" ADD CONSTRAINT session_hall_id FOREIGN KEY ("hall_id") REFERENCES "dashboard"."hall" ("id");

ALTER TABLE "order" ADD CONSTRAINT order_session_id FOREIGN KEY ("session_id") REFERENCES "session" ("id");
