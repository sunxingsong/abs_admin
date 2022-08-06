use crate::domain::domain::LoginCheck;
use rbdc::datetime::FastDateTime;

///权限资源表
#[derive(Clone, Debug)]
pub struct SysRes {
    pub id: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //权限
    pub permission: Option<String>,
    //前端-菜单路径
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

impl_field_name_method!(SysRes{id,parent_id,name,permission,path,del,create_date});

///角色表
#[derive(Clone, Debug)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //父id(可空)
    pub parent_id: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

impl_field_name_method!(SysRole{id,parent_id,name,del,create_date});

///角色资源关系表(关系表不使用逻辑删除)

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SysRoleRes {
    pub id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    //资源id
    pub res_id: Option<String>,
    pub create_date: Option<FastDateTime>,
}

impl_field_name_method!(SysRoleRes{id,role_id,res_id,create_date});

///后台用户表

#[derive(Clone, Debug)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

impl_field_name_method!(SysUser{id,account,password,name,login_check,state,del,create_date});

///用户角色关系表(关系表不使用逻辑删除)

#[derive(Clone, Debug)]
pub struct SysUserRole {
    pub id: Option<String>,
    //用户id
    pub user_id: Option<String>,
    //角色id
    pub role_id: Option<String>,
    pub create_date: Option<FastDateTime>,
}
impl_field_name_method!(SysUserRole{id,user_id,role_id,create_date});
///字典表

#[derive(Clone, Debug)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<FastDateTime>,
}
impl_field_name_method!(SysDict{id,name,code,state,create_date});