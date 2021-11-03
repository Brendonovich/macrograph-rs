use macrograph::core::Core;

pub fn load_packages(core: &mut Core) {
    let packages = macrograph_packages::create_packages();
    
    packages.into_iter().for_each(|p| core.register_package(p));
}