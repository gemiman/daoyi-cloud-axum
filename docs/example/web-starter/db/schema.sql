SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

CREATE DATABASE IF NOT EXISTS demo DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE demo;

drop table if exists demo_sys_user;
CREATE TABLE IF NOT EXISTS demo_sys_user
(
    id            BIGINT                                                       NOT NULL AUTO_INCREMENT,
    name          VARCHAR(16)                                                  NOT NULL,
    gender        VARCHAR(8)                                                   NOT NULL,
    account       VARCHAR(16)                                                  NOT NULL,
    password      VARCHAR(64)                                                  NOT NULL,
    mobile_phone  VARCHAR(16)                                                  NOT NULL,
    birthday      DATE                                                         NOT NULL,
    enabled       BOOLEAN                                                               DEFAULT TRUE NOT NULL,
    `creator`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL     DEFAULT '' COMMENT '创建者',
    `create_time` datetime                                                     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updater`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL     DEFAULT '' COMMENT '更新者',
    `update_time` datetime                                                     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted`     bit(1)                                                       NOT NULL DEFAULT b'0' COMMENT '是否删除',
    `tenant_id`   bigint                                                       NOT NULL DEFAULT 0 COMMENT '租户编号',
    PRIMARY KEY (`id`) USING BTREE
);

INSERT INTO demo_sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled)
VALUES (1, '李四', 'female', 'lisi', '$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK',
        '17361631996', '2025-05-13', TRUE);
INSERT INTO demo_sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled)
VALUES (2, '张三', 'male', 'admin', '$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK',
        '19909407240', '2025-05-18', FALSE);
INSERT INTO demo_sys_user (id, name, gender, account, password, mobile_phone, birthday, enabled)
VALUES (3, '赵六', 'female', 'zhaoliu', '$2b$12$EJOKHLJLnfHrgrXbZl8uge3N4VEgR9FWHwq3a6pgTIM8O66Lf/9DW',
        '18361631783', '2025-06-11', TRUE);

commit;


-- ----------------------------
-- Table structure for demo_contact
-- ----------------------------
DROP TABLE IF EXISTS `demo_contact`;
CREATE TABLE `demo_contact`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT COMMENT '编号',
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名字',
    `sex`         tinyint(1)                                                    NOT NULL COMMENT '性别',
    `birthday`    datetime                                                      NOT NULL COMMENT '出生年',
    `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '简介',
    `avatar`      varchar(512) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL     DEFAULT NULL COMMENT '头像',
    `creator`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '创建者',
    `create_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updater`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '更新者',
    `update_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted`     bit(1)                                                        NOT NULL DEFAULT b'0' COMMENT '是否删除',
    `tenant_id`   bigint                                                        NOT NULL DEFAULT 0 COMMENT '租户编号',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  AUTO_INCREMENT = 2
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_unicode_ci COMMENT = '示例联系人表';

-- ----------------------------
-- Records of demo_contact
-- ----------------------------
BEGIN;
INSERT INTO `demo_contact` (`id`, `name`, `sex`, `birthday`, `description`, `avatar`, `creator`, `create_time`,
                            `updater`, `update_time`, `deleted`, `tenant_id`)
VALUES (1, '土豆', 2, '2023-11-07 00:00:00', '<p>天蚕土豆！呀</p>',
        'http://127.0.0.1:48080/admin-api/infra/file/4/get/46f8fa1a37db3f3960d8910ff2fe3962ab3b2db87cf2f8ccb4dc8145b8bdf237.jpeg',
        '1', '2023-11-15 23:34:30', '1', '2023-11-15 23:47:39', b'0', 1);
COMMIT;

-- ----------------------------
-- Table structure for demo_category
-- ----------------------------
DROP TABLE IF EXISTS `demo_category`;
CREATE TABLE `demo_category`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT COMMENT '编号',
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名字',
    `parent_id`   bigint                                                        NOT NULL COMMENT '父级编号',
    `creator`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '创建者',
    `create_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updater`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '更新者',
    `update_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted`     bit(1)                                                        NOT NULL DEFAULT b'0' COMMENT '是否删除',
    `tenant_id`   bigint                                                        NOT NULL DEFAULT 0 COMMENT '租户编号',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  AUTO_INCREMENT = 8
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_unicode_ci COMMENT = '示例分类表';

-- ----------------------------
-- Records of demo_category
-- ----------------------------
BEGIN;
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (1, '土豆', 0, '1', '2023-11-15 23:34:30', '1', '2023-11-16 20:24:23', b'0', 1);
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (2, '番茄', 0, '1', '2023-11-16 20:24:00', '1', '2023-11-16 20:24:15', b'0', 1);
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (3, '怪怪', 0, '1', '2023-11-16 20:24:32', '1', '2023-11-16 20:24:32', b'0', 1);
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (4, '小番茄', 2, '1', '2023-11-16 20:24:39', '1', '2023-11-16 20:24:39', b'0', 1);
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (5, '大番茄', 2, '1', '2023-11-16 20:24:46', '1', '2023-11-16 20:24:46', b'0', 1);
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (6, '11', 3, '1', '2023-11-24 19:29:34', '1', '2023-11-24 19:29:34', b'0', 1);
INSERT INTO `demo_category` (`id`, `name`, `parent_id`, `creator`, `create_time`, `updater`, `update_time`, `deleted`,
                             `tenant_id`)
VALUES (7, '1', 0, '1', '2025-10-01 09:19:20', '1', '2025-10-01 09:19:20', b'0', 1);
COMMIT;

-- ----------------------------
-- Table structure for demo_course
-- ----------------------------
DROP TABLE IF EXISTS `demo_course`;
CREATE TABLE `demo_course`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT COMMENT '编号',
    `student_id`  bigint                                                        NOT NULL COMMENT '学生编号',
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名字',
    `score`       tinyint                                                       NOT NULL COMMENT '分数',
    `creator`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '创建者',
    `create_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updater`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '更新者',
    `update_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted`     bit(1)                                                        NOT NULL DEFAULT b'0' COMMENT '是否删除',
    `tenant_id`   bigint                                                        NOT NULL DEFAULT 0 COMMENT '租户编号',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  AUTO_INCREMENT = 22
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_unicode_ci COMMENT = '学生课程表';

-- ----------------------------
-- Records of demo_course
-- ----------------------------
BEGIN;
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (2, 2, '语文', 66, '1', '2023-11-16 23:21:49', '1', '2024-09-17 10:55:30', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (3, 2, '数学', 22, '1', '2023-11-16 23:21:49', '1', '2024-09-17 10:55:30', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (6, 5, '体育', 23, '1', '2023-11-16 23:22:46', '1', '2023-11-16 15:44:40', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (7, 5, '计算机', 11, '1', '2023-11-16 23:22:46', '1', '2023-11-16 15:44:40', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (8, 5, '体育', 23, '1', '2023-11-16 23:22:46', '1', '2023-11-16 15:47:09', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (9, 5, '计算机', 11, '1', '2023-11-16 23:22:46', '1', '2023-11-16 15:47:09', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (10, 5, '体育', 23, '1', '2023-11-16 23:22:46', '1', '2024-09-17 10:55:28', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (11, 5, '计算机', 11, '1', '2023-11-16 23:22:46', '1', '2024-09-17 10:55:28', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (12, 2, '电脑', 33, '1', '2023-11-17 00:20:42', '1', '2023-11-16 16:20:45', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (13, 9, '滑雪', 12, '1', '2023-11-17 13:13:20', '1', '2024-09-17 10:55:26', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (14, 9, '滑雪', 12, '1', '2023-11-17 13:13:20', '1', '2024-09-17 10:55:49', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (15, 5, '体育', 23, '1', '2023-11-16 23:22:46', '1', '2024-09-17 18:55:29', b'0', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (16, 5, '计算机', 11, '1', '2023-11-16 23:22:46', '1', '2024-09-17 18:55:29', b'0', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (17, 2, '语文', 66, '1', '2023-11-16 23:21:49', '1', '2024-09-17 18:55:31', b'0', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (18, 2, '数学', 22, '1', '2023-11-16 23:21:49', '1', '2024-09-17 18:55:31', b'0', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (19, 9, '滑雪', 12, '1', '2023-11-17 13:13:20', '1', '2025-04-19 02:49:03', b'1', 1);
INSERT INTO `demo_course` (`id`, `student_id`, `name`, `score`, `creator`, `create_time`, `updater`, `update_time`,
                           `deleted`, `tenant_id`)
VALUES (20, 9, '滑雪', 12, '1', '2023-11-17 13:13:20', '1', '2025-04-19 10:49:04', b'0', 1);
COMMIT;

-- ----------------------------
-- Table structure for demo_grade
-- ----------------------------
DROP TABLE IF EXISTS `demo_grade`;
CREATE TABLE `demo_grade`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT COMMENT '编号',
    `student_id`  bigint                                                        NOT NULL COMMENT '学生编号',
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名字',
    `teacher`     varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '班主任',
    `creator`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '创建者',
    `create_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updater`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '更新者',
    `update_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted`     bit(1)                                                        NOT NULL DEFAULT b'0' COMMENT '是否删除',
    `tenant_id`   bigint                                                        NOT NULL DEFAULT 0 COMMENT '租户编号',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  AUTO_INCREMENT = 10
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_unicode_ci COMMENT = '学生班级表';

-- ----------------------------
-- Records of demo_grade
-- ----------------------------
BEGIN;
INSERT INTO `demo_grade` (`id`, `student_id`, `name`, `teacher`, `creator`, `create_time`, `updater`, `update_time`,
                          `deleted`, `tenant_id`)
VALUES (7, 2, '三年 2 班', '周杰伦', '1', '2023-11-16 23:21:49', '1', '2024-09-17 18:55:31', b'0', 1);
INSERT INTO `demo_grade` (`id`, `student_id`, `name`, `teacher`, `creator`, `create_time`, `updater`, `update_time`,
                          `deleted`, `tenant_id`)
VALUES (8, 5, '华为', '遥遥领先', '1', '2023-11-16 23:22:46', '1', '2024-09-17 18:55:29', b'0', 1);
INSERT INTO `demo_grade` (`id`, `student_id`, `name`, `teacher`, `creator`, `create_time`, `updater`, `update_time`,
                          `deleted`, `tenant_id`)
VALUES (9, 9, '小图', '小娃111', '1', '2023-11-17 13:10:23', '1', '2025-04-19 10:49:04', b'0', 1);
COMMIT;

-- ----------------------------
-- Table structure for demo_student
-- ----------------------------
DROP TABLE IF EXISTS `demo_student`;
CREATE TABLE `demo_student`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT COMMENT '编号',
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '名字',
    `sex`         tinyint                                                       NOT NULL COMMENT '性别',
    `birthday`    datetime                                                      NOT NULL COMMENT '出生日期',
    `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '简介',
    `creator`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '创建者',
    `create_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updater`     varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci  NULL     DEFAULT '' COMMENT '更新者',
    `update_time` datetime                                                      NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    `deleted`     bit(1)                                                        NOT NULL DEFAULT b'0' COMMENT '是否删除',
    `tenant_id`   bigint                                                        NOT NULL DEFAULT 0 COMMENT '租户编号',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  AUTO_INCREMENT = 10
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_unicode_ci COMMENT = '学生表';

-- ----------------------------
-- Records of demo_student
-- ----------------------------
BEGIN;
INSERT INTO `demo_student` (`id`, `name`, `sex`, `birthday`, `description`, `creator`, `create_time`, `updater`,
                            `update_time`, `deleted`, `tenant_id`)
VALUES (2, '小白', 1, '2023-11-16 00:00:00', '<p>厉害</p>', '1', '2023-11-16 23:21:49', '1', '2024-09-17 18:55:31',
        b'0', 1);
INSERT INTO `demo_student` (`id`, `name`, `sex`, `birthday`, `description`, `creator`, `create_time`, `updater`,
                            `update_time`, `deleted`, `tenant_id`)
VALUES (5, '大黑', 2, '2023-11-13 00:00:00', '<p>你在教我做事?</p>', '1', '2023-11-16 23:22:46', '1',
        '2024-09-17 18:55:29', b'0', 1);
INSERT INTO `demo_student` (`id`, `name`, `sex`, `birthday`, `description`, `creator`, `create_time`, `updater`,
                            `update_time`, `deleted`, `tenant_id`)
VALUES (9, '小花', 1, '2023-11-07 00:00:00', '<p>哈哈哈</p>', '1', '2023-11-17 00:04:47', '1', '2025-04-19 10:49:04',
        b'0', 1);
COMMIT;

SET FOREIGN_KEY_CHECKS = 1;