Pod::Spec.new do |s|  
    s.name              = 'WnfsBindings' # Name for your pod
    s.version           = '0.1.0'
    s.summary           = 'Swift bindings for the rust WNFS library'
    s.homepage          = 'https://github.com/hhio618/wnfs-build-xcframework'

    s.author            = { 'Homayoun Heidarzadeh' => 'hhio618@gmail.com' }
    s.license = "MIT (hhio618)"

    s.platform          = :ios
    # change the source location
    s.source            = { :git => './' } 
    s.ios.source_files = "include/*.{h}"
    s.ios.deployment_target = '11.0'
    s.ios.vendored_libraries = '*.a'
    s.static_framework = true
end 
