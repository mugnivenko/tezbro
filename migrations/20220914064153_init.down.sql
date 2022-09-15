ALTER TABLE "dashboard"."cinema" DROP CONSTRAINT dashboard_cinema_admin_id;

ALTER TABLE "dashboard"."photo" DROP CONSTRAINT dashboard_photo_cinema_id;

ALTER TABLE "dashboard"."hall" DROP CONSTRAINT dashboard_hall_cinema_id;

ALTER TABLE "dashboard"."seat" DROP CONSTRAINT dashboard_seat_hall_id;

ALTER TABLE "dashboard"."password" DROP CONSTRAINT dashboard_password_user_id;

ALTER TABLE "session" DROP CONSTRAINT session_movie_id;

ALTER TABLE "session" DROP CONSTRAINT session_hall_id;

ALTER TABLE "order" DROP CONSTRAINT order_session_id;

DROP TABLE "dashboard"."cinema";

DROP TABLE "dashboard"."photo";

DROP TABLE "dashboard"."hall";

DROP TABLE "dashboard"."seat";

DROP TABLE "dashboard"."user";

DROP TABLE "dashboard"."password";

DROP TABLE "movie";

DROP TABLE "session";

DROP TABLE "order";

DROP TYPE "dashboard"."role";

DROP SCHEMA "dashboard";
