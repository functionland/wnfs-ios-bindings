Pod::Spec.new do |s|  
    s.name              = 'WnfsBindings' # Name for your pod
    s.version           = '0.1.0'
    s.summary           = 'Swift bindings for the rust WNFS library'
    s.homepage          = 'https://github.com/functionland/wnfs-swift-bindings'

    s.author            = { 'Homayoun Heidarzadeh' => 'hhio618@gmail.com' }
    s.license = "MIT (functionland)"

    s.platform          = :ios
    # change the source location
    s.source            = { :http => 'https://github.com/functionland/wnfs-swift-bindings/releases/download/v0.1.2/swift-bundle.zip' } 
    s.source_files = "include/*.{h}"
    s.ios.deployment_target = '11.0'
    # s.ios.vendored_libraries = 'libwnfsbindings_iossimulator.a'
    # s.vendored_libraries = 'libwnfsbindings_ios.a'
    s.vendored_frameworks = 'WnfsBindings.xcframework'
    s.static_framework = true
end 
