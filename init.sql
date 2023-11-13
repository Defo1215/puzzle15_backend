CREATE TABLE user
(
    `id`         INT(11) NOT NULL PRIMARY KEY COMMENT 'ID',
    `username`   VARCHAR(255) NOT NULL COMMENT '用户名',
    `password`   VARCHAR(255) NOT NULL COMMENT '密码 md5',
    `nickname`   VARCHAR(255) NOT NULL COMMENT '昵称',
    `email`      VARCHAR(255)          DEFAULT '' COMMENT '邮箱',
    `phone`      VARCHAR(255)          DEFAULT '' COMMENT '电话',
    `avatar`     VARCHAR(255)          DEFAULT '' COMMENT '头像',
    `status`     INT(11) NOT NULL DEFAULT 0 COMMENT '状态 0 正常 1 删除 2 冻结',
    `created_at` TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    UNIQUE KEY `uniq_username` (`username`),
    UNIQUE KEY `uniq_phone` (`phone`),
    UNIQUE KEY `uniq_email` (`email`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 COMMENT ='用户表';


CREATE TABLE record
(
    `id`         INT(11) NOT NULL PRIMARY KEY COMMENT 'ID',
    `user_id`    INT(11) NOT NULL COMMENT '用户ID',
    `event`      varchar(32) NOT NULL COMMENT '项目 333,444,555,p3,p8,p15',
    `step_count` INT(11) NOT NULL COMMENT '步数',
    `duration`   INT(11) NOT NULL COMMENT '时长 ms',
    `status`     INT(11) NOT NULL DEFAULT 0 COMMENT '状态 0 正常 1 删除',
    `scramble`   TEXT        NOT NULL COMMENT '打乱',
    `solution`   TEXT        NOT NULL COMMENT '解法',
    `created_at` TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    KEY          `idx_user_id` (`user_id`),
    KEY          `idx_event` (`event`),
    KEY          `idx_user_event` (`user_id`, `event`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 COMMENT ='记录表';
