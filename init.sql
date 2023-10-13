create table TestDB.user
(
    id            bigint       not null auto_increment,
    username      varchar(255) not null,
    password      varchar(255) not null,
    register_time timestamp    not null default current_timestamp,
    update_time   timestamp    not null on update current_timestamp default current_timestamp,
    delete_time   timestamp null default null,
    constraint id primary key (id)
) engine=InnoDB auto_increment=1000 default charset=utf8mb4;

create table TestDB.task
(
    id          bigint       not null auto_increment,
    userid      bigint       not null,
    title       varchar(255) not null,
    content     varchar(255) not null,
    finished    char(0) null default null, -- 将''视为true NULL视为false --
    start_time  timestamp    not null,
    end_time    timestamp    null,
    create_time timestamp    not null default current_timestamp,
    update_time timestamp    not null on update current_timestamp default current_timestamp,
    delete_time timestamp null default null,
    constraint id primary key (id)
) engine=InnoDB auto_increment=1000 default charset=utf8mb4;

