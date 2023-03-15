Pod::Spec.new do |s|  
    s.name              = 'WnfsBindings' # Name for your pod
    s.version           = '0.1.6'
    s.summary           = 'Swift bindings for the rust WNFS library'
    s.homepage          = 'https://github.com/functionland/wnfs-swift-bindings'

    s.author            = { 'Homayoun Heidarzadeh' => 'hhio618@gmail.com' }
    s.license = { :type => 'MIT', :file => 'LICENSE' }

    s.platform          = :ios
    # change the source location
    s.source            = { :http => "https://github.com/functionland/wnfs-swift-bindings/releases/download/v#{s.version}/cocoapods-bundle.zip" } 
    s.source_files = "include/*.{h}"
    s.module_map = "include/module.modulemap"
    s.ios.deployment_target = '11.0'
    s.ios.vendored_libraries = 'libwnfsbindings_iossimulator.a'
    s.osx.vendored_libraries = 'libwnfsbindings_darwin.a'
    s.static_framework = true
    s.user_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
    s.pod_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'arm64' }
end 
