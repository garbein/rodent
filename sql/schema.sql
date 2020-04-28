CREATE DATABASE rodent;
USE rodent;
CREATE TABLE `site_setting` (
  `id` int unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `name` varchar(255) NOT NULL DEFAULT ''  COMMENT '配置名称',
  `title` varchar(255) NOT NULL DEFAULT '' COMMENT '配置标题',
  `content` longtext NOT NULL  COMMENT '配置内容',
  `status` tinyint NOT NULL DEFAULT '0' COMMENT '状态',
  `mtime` int unsigned NOT NULL DEFAULT '0' COMMENT '修改时间',
  `ctime` int unsigned NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint NOT NULL DEFAULT '0' COMMENT '标记删除',
  PRIMARY KEY (`id`),
  KEY `idx_name` (`name`)
) ENGINE=InnoDB;