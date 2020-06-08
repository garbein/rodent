CREATE DATABASE rodent;
USE rodent;
CREATE TABLE `site_setting` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `name` varchar(255) NOT NULL DEFAULT ''  COMMENT '配置名称',
  `title` varchar(255) NOT NULL DEFAULT '' COMMENT '配置标题',
  `content` longtext NOT NULL  COMMENT '配置内容',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '状态',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '修改时间',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '标记删除',
  PRIMARY KEY (`id`),
  KEY `idx_name` (`name`)
) ENGINE=InnoDB;

CREATE TABLE `user` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `name` varchar(255) NOT NULL DEFAULT ''  COMMENT '用户名',
  `avatar` varchar(255) NOT NULL DEFAULT '' COMMENT '用户头像'
  `email` varchar(255) NOT NULL DEFAULT '' COMMENT '邮箱',
  `password` varchar(255) NOT NULL DEFAULT '' COMMENT '密码',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '状态',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '修改时间',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '标记删除',
  PRIMARY KEY (`id`),
  KEY `idx_email` (`email`)
) ENGINE=InnoDB;

CREATE TABLE `task` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `name` varchar(255) NOT NULL DEFAULT ''  COMMENT '任务名称',
  `type` tinyint NOT NULL DEFAULT '0' COMMENT '任务类型',
  `remark` longtext NOT NULL COMMENT '备注信息',
  `icon_bean` longtext NOT NULL COMMENT '图标配置',
  `detail_list` longtext NOT NULL COMMENT '项目条目',
  `detail_num` int NOT NULL DEFAULT '0' COMMENT '项目条数',
  `change_times` int NOT NULL DEFAULT '0' COMMENT '修改次数',
  `progress` tinyint NOT NULL DEFAULT '0' COMMENT '进度',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '状态',
  `start_at` int NOT NULL DEFAULT '0' COMMENT '开始时间',
  `end_at` int NOT NULL DEFAULT '0' COMMENT '结束时间',
  `finished_at` int NOT NULL DEFAULT '0' COMMENT '完成时间',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '修改时间',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '标记删除',
  PRIMARY KEY (`id`),
  KEY `idx_name` (`name`)
) ENGINE=InnoDB;

CREATE TABLE `suggestion` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `account` varchar(255) NOT NULL DEFAULT ''  COMMENT '账号',
  `user_avatar` varchar(128) NOT NULL DEFAULT '' COMMENT '用户头像',
  `user_name` varchar(128) NOT NULL DEFAULT '' COMMENT '用户名',
  `emotion` varchar(64) NOT NULL DEFAULT '' COMMENT '心情',
  `contact` varchar(128) NOT NULL DEFAULT '' COMMENT '联系方式',
  `content` longtext NOT NULL COMMENT '建议内容',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '状态',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '修改时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '标记删除',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB;